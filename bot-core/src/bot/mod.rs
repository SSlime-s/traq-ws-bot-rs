use std::any::Any;

use arrayvec::ArrayVec;
use futures::{future, StreamExt};
use paste::paste;
use reqwest::Url;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{handshake::client::generate_key, Message},
};

use crate::events::{convert_handler, payload, Events};

pub mod keys;

pub type Handler = Box<dyn Fn(&Events)>;

pub type OnPanic = Box<dyn Any + Send>;
pub type OnPanicHandler = Box<dyn Fn(OnPanic)>;

pub const TRAQ_ORIGIN: &str = "https://q.trap.jp";
pub const TRAQ_ORIGIN_WS: &str = "wss://q.trap.jp";

pub const TRAQ_WS_GATEWAY_PATH: &str = "/api/v3/bot/ws";

pub struct TraqBotBuilder {
    authorization_scheme: String,
    token: String,
    target_url: Url,
    handlers: ArrayVec<Vec<Handler>, { keys::KEYS_COUNT }>,
    on_handler_panic: Option<OnPanicHandler>,
}

pub struct TraqBot {
    authorization_scheme: String,
    token: String,
    ws_origin: Url,
    gateway_path: String,
    handlers: ArrayVec<Box<[Handler]>, { keys::KEYS_COUNT }>,
    on_handler_panic: Option<OnPanicHandler>,
}

macro_rules! on_x_payload {
    ($($x:ident),*$(,)?) => {
        $(
            paste! {
                #[doc = ""[<$x:camel>]" イベントを受け取った際のハンドラを登録する"]
                #[doc = ""]
                #[doc = "# Example"]
                #[doc = "```rust"]
                #[doc = "use traq_ws_bot::bot::builder;"]
                #[doc = ""]
                #[doc = "let bot = builder(\"BOT_ACCESS_TOKEN\")"]
                #[doc = "    ."[<on_ $x:snake>]"(|event| {"]
                #[doc = "        println!(\"{:?}\", event);"]
                #[doc = "    })"]
                #[doc = "    .build();"]
                #[doc = "```"]
                pub fn [<on_ $x:snake>]<F: Fn(&payload::[<$x:camel>]) + 'static>(mut self, handler: F) -> Self {
                    let handler = convert_handler!(handler => [<$x:camel>]);
                    self.handlers[keys::Keys::[<$x:camel>] as usize].push(Box::new(handler));
                    self
                }
                #[doc = [<$x:camel>] イベントを受け取った際のハンドラを複数同時に登録する]
                #[doc(hidden)]
                pub fn [<on_ $x:snake _multi>]<F: Fn(&payload::[<$x:camel>]) + 'static, Fs: Into<Vec<F>>>(mut self, handlers: Fs) -> Self {
                    let handlers: Vec<_> = handlers.into();
                    for handler in handlers {
                        let handler = convert_handler!(handler => [<$x:camel>]);
                        self.handlers[keys::Keys::[<$x:camel>] as usize].push(Box::new(handler));
                    }
                    self
                }
            }
        )*
    };
}

macro_rules! handle_event_inner {
    ($self:expr, $event:expr => {$($x:ident),*$(,)?}) => {
        paste!{
            match $event {
                $(
                    Events::[<$x:camel>](_) => {
                        for handler in $self.handlers[keys::Keys::[<$x:camel>] as usize].iter() {
                            if let Some(on_panic) = &$self.on_handler_panic {
                                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    handler(&$event);
                                }));
                                if let Err(err) = result {
                                    on_panic(err)
                                }
                            } else {
                                handler(&$event);
                            }
                        }
                    }
                )*
            }
        }
    }
}

impl TraqBot {
    /// BOT を起動する
    ///
    /// # Examples
    /// ```rust
    /// use traq_ws_bot::bot::builder;
    ///
    /// # async fn try_main() -> anyhow::Result<()> {
    /// let bot = builder("BOT_ACCESS_TOKEN")
    ///     .on_message_created(|event| {
    ///       println!("{:?}", event);
    ///     })
    ///     .build();
    /// bot.start().await?;
    /// # Ok(())
    /// # }
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// #     let _ = try_main().await;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn start(&self) -> anyhow::Result<()> {
        let host = self.get_ws_url().host_str().unwrap().to_owned();
        let request = http::Request::builder()
            .method("GET")
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-Websocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .uri(self.get_ws_url().to_string())
            .header(
                "Authorization",
                format!("{} {}", self.authorization_scheme, self.token),
            )
            .body(())?;

        let (ws_stream, _) = connect_async(request).await?;

        let (_tx, rx) = futures::channel::mpsc::unbounded();
        let (write, read) = ws_stream.split();

        let write_loop = rx.map(Ok).forward(write);

        let read_loop = {
            read.for_each(|message| async {
                match message {
                    Ok(message) => {
                        match message {
                            Message::Ping(_) => {
                                // nop
                            }
                            Message::Text(content) => {
                                let event = serde_json::from_str(&content);
                                if let Ok(event) = event {
                                    self.handle_event(&event).await;
                                } else {
                                    eprintln!("failed to parse event: {}", content);
                                }
                            }
                            _ => {
                                eprintln!("not supported message: {:?}", message);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("error: {:?}", e);
                    }
                }
            })
        };

        let (write_res, read_res) = future::join(write_loop, read_loop).await;
        let (_write_res, _read_res) = (write_res?, read_res);

        Ok(())
    }

    /// ws もしくは wss で始まる origin に相当する URL を返す
    ///
    /// **Example** `wss://q.trap.jp`, `ws://localhost:8080`
    pub fn get_ws_origin(&self) -> Url {
        self.ws_origin.clone()
    }
    /// http もしくは https で始まる origin に相当する URL を返す
    ///
    /// **Example** `https://q.trap.jp`, `http://localhost:8080`
    pub fn get_http_origin(&self) -> Url {
        let mut origin = self.get_ws_origin();
        match origin.scheme() {
            "wss" => origin.set_scheme("https").unwrap(),
            "ws" => origin.set_scheme("http").unwrap(),
            _ => panic!("Invalid scheme: {} (expected: ws, wss)", origin.scheme()),
        }
        origin
    }

    /// ws もしくは wss で始まる gateway の URL を返す
    ///
    /// **Example** `wss://q.trap.jp/api/v3/bot/ws`
    pub fn get_ws_url(&self) -> Url {
        self.ws_origin.join(&self.gateway_path).unwrap()
    }
    /// http もしくは https で始まる gateway の URL を返す
    ///
    /// **Example** `https://q.trap.jp/api/v3/bot/ws`
    pub fn get_http_url(&self) -> Url {
        let mut url = self.get_ws_url();
        match url.scheme() {
            "wss" => url.set_scheme("https").unwrap(),
            "ws" => url.set_scheme("http").unwrap(),
            _ => panic!("Invalid scheme: {} (expected: ws, wss)", url.scheme()),
        }
        url
    }

    /// bot access token を返す
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// イベントに対してハンドラを呼び出す
    async fn handle_event(&self, event: &Events) {
        handle_event_inner!(
            self,
            event => {
                Ping,
                Joined,
                Left,
                MessageCreated,
                MessageUpdated,
                MessageDeleted,
                BotMessageStampsUpdated,
                DirectMessageCreated,
                DirectMessageUpdated,
                DirectMessageDeleted,
                ChannelCreated,
                ChannelTopicChanged,
                UserCreated,
                StampCreated,
                TagAdded,
                TagRemoved,
                Error,
            }
        )
    }
}

/// TraqBot の Builder を作成する
pub fn builder(token: impl Into<String>) -> TraqBotBuilder {
    TraqBotBuilder {
        token: token.into(),
        ..Default::default()
    }
}

#[doc(hidden)]
#[allow(unused)]
#[rustfmt::skip]
/*pub */ fn builder_with_config(_config: ()) -> TraqBotBuilder {
    unimplemented!()
}

impl Default for TraqBotBuilder {
    fn default() -> Self {
        let handlers_arr: [Vec<_>; keys::KEYS_COUNT] = Default::default();

        Self {
            authorization_scheme: "Bearer".to_owned(),
            token: Default::default(),
            target_url: Url::parse(TRAQ_ORIGIN_WS)
                .unwrap()
                .join(TRAQ_WS_GATEWAY_PATH)
                .unwrap(),
            handlers: ArrayVec::from(handlers_arr),
            on_handler_panic: Some(Box::new(|e| {
                eprintln!("{:?}", e);
            })),
        }
    }
}

fn convert_to_ws_url<U>(url: U) -> anyhow::Result<Url>
where
    U: TryInto<Url>,
    U::Error: std::error::Error + Send + Sync + 'static,
{
    let mut url = url.try_into()?;
    match url.scheme() {
        "wss" | "ws" => Ok(url),
        "http" => {
            url.set_scheme("ws").unwrap();
            Ok(url)
        }
        "https" => {
            url.set_scheme("wss").unwrap();
            Ok(url)
        }
        _ => Err(anyhow::anyhow!(
            "Invalid scheme: {} (expected: ws, wss, http, https)",
            url.scheme()
        )),
    }
}

impl TraqBotBuilder {
    /// TraqBotBuilder から TraqBot を作成する
    ///
    /// # Example
    /// ```
    /// use traq_ws_bot::bot::builder;
    ///
    /// let bot = builder("BOT_ACCESS_TOKEN")
    ///     .on_message_created(|event| {
    ///         println!("{:?}", event);
    ///     })
    ///    .build();
    /// ```
    pub fn build(self) -> TraqBot {
        let target_url_ws = convert_to_ws_url(self.target_url).unwrap();
        let ws_origin = target_url_ws
            .origin()
            .ascii_serialization()
            .parse()
            .unwrap();
        let gateway_path = target_url_ws.path().to_owned();

        TraqBot {
            authorization_scheme: self.authorization_scheme,
            token: self.token,
            ws_origin,
            gateway_path,
            handlers: self
                .handlers
                .into_iter()
                .map(|v| v.into_boxed_slice())
                .collect(),
            on_handler_panic: self.on_handler_panic,
        }
    }

    /// 認証の scheme を指定する
    ///
    /// **Default** `Bearer`
    pub fn set_auth_scheme(mut self, scheme: impl Into<String>) -> Self {
        self.authorization_scheme = scheme.into();
        self
    }
    /// Bot の access token を指定する
    pub fn set_token(mut self, token: impl Into<String>) -> Self {
        self.token = token.into();
        self
    }
    /// Bot が参加するための WebSocket の URL を指定する
    ///
    /// **Default** `wss://q.trap.jp/api/v3/bot/ws`
    pub fn set_target_url(mut self, url: impl Into<Url>) -> Self {
        self.target_url = url.into();
        self
    }

    /// 登録したハンドラが panic した際のハンドラを設定する
    ///
    /// **Warning**: このハンドラが panic した場合、プログラムが終了します
    ///
    /// # Example
    /// ```rust
    /// use traq_ws_bot::bot::builder;
    ///
    /// let bot = builder("BOT_ACCESS_TOKEN")
    ///     .set_on_panic_handler(|e| {
    ///         eprintln!("handler is panicked: {:?}", e);
    ///     })
    ///    .build();
    /// ```
    pub fn set_on_panic_handler<F: Fn(OnPanic) + 'static>(mut self, handler: F) -> Self {
        self.on_handler_panic = Some(Box::new(handler));
        self
    }

    /// key のイベントに対応するハンドラを設定する
    ///
    /// ハンドラに渡される enum は key で指定したイベントであることが保証される
    ///
    /// # Example
    /// ```rust
    /// use traq_ws_bot::{bot::{builder, keys::Keys}, events::Events};
    ///
    /// let bot = builder("BOT_ACCESS_TOKEN")
    ///     .on_event(Keys::Joined, |event| {
    ///        if let Events::Joined(event) = event {
    ///           println!("{:?}", event);
    ///       }
    ///    })
    ///   .build();
    /// ```
    pub fn on_event<F: Fn(&Events) + 'static>(mut self, key: keys::Keys, handler: F) -> Self {
        self.handlers[key as usize].push(Box::new(handler));
        self
    }
    /// key のイベントに対応するハンドラを複数同時に設定する
    ///
    /// ハンドラに渡される enum は key で指定したイベントであることが保証される
    #[doc(hidden)]
    pub fn on_event_multi<F: Fn(&Events) + 'static, Fs: Into<Vec<F>>>(
        mut self,
        key: keys::Keys,
        handlers: Fs,
    ) -> Self {
        let handlers: Vec<_> = handlers.into();
        for handler in handlers {
            self.handlers[key as usize].push(Box::new(handler));
        }
        self
    }

    on_x_payload!(
        Ping,
        Joined,
        Left,
        MessageCreated,
        MessageUpdated,
        MessageDeleted,
        BotMessageStampsUpdated,
        DirectMessageCreated,
        DirectMessageUpdated,
        DirectMessageDeleted,
        ChannelCreated,
        ChannelTopicChanged,
        UserCreated,
        StampCreated,
        TagAdded,
        TagRemoved,
    );

    #[doc = "Error イベントを受け取った際のハンドラを登録する"]
    #[doc = ""]
    #[doc = "# Example"]
    #[doc = "```rust"]
    #[doc = "use traq_ws_bot::bot::builder;"]
    #[doc = ""]
    #[doc = "let bot = builder(\"BOT_ACCESS_TOKEN\")"]
    #[doc = "    .on_error(|event| {"]
    #[doc = "        println!(\"{:?}\", event);"]
    #[doc = "    })"]
    #[doc = "    .build();"]
    #[doc = "```"]
    pub fn on_error<F: Fn(&str) + 'static>(mut self, handler: F) -> Self {
        let handler = convert_handler!(handler => Error);
        self.handlers[keys::Keys::Error as usize].push(Box::new(handler));
        self
    }
    #[doc = "Error イベントを受け取った際のハンドラを複数同時に登録する"]
    #[doc(hidden)]
    pub fn on_error_multi<F: Fn(&str) + 'static, Fs: Into<Vec<F>>>(mut self, handlers: Fs) -> Self {
        let handlers: Vec<_> = handlers.into();
        for handler in handlers {
            let handler = convert_handler!(handler => Error);
            self.handlers[keys::Keys::Error as usize].push(Box::new(handler));
        }
        self
    }
}
