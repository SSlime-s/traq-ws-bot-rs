use std::{any::Any, collections::HashMap, fmt::Debug};

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

pub struct TraqBot {
    bear_token: String,
    ws_origin: Url,
    gateway_path: String,
    handlers: HashMap<keys::Keys, Vec<Handler>>,
    on_handler_panic: OnPanicHandler,
}

macro_rules! on_x_payload {
    ($($x:ident),*$(,)?) => {
        $(
            paste! {
                #[doc = [<$x:camel>] イベントを受け取った際のハンドラを登録する]
                pub fn [<on_ $x:snake>]<F: Fn(&payload::[<$x:camel>]) + 'static>(mut self, handler: F) -> Self {
                    let handler = convert_handler!(handler => [<$x:camel>]);
                    self.handlers
                        .entry(keys::Keys::[<$x:camel>])
                        .or_insert(vec![])
                        .push(Box::new(handler));
                    self
                }
                #[doc = [<$x:camel>] イベントを受け取った際のハンドラを複数同時に登録する]
                pub fn [<on_ $x:snake _multi>]<F: Fn(&payload::[<$x:camel>]) + 'static, Fs: Into<Vec<F>>>(mut self, handlers: Fs) -> Self {
                    let handlers: Vec<_> = handlers.into();
                    let entry = self.handlers.entry(keys::Keys::[<$x:camel>]).or_insert(vec![]);
                    for handler in handlers {
                        let handler = convert_handler!(handler => [<$x:camel>]);
                        entry.push(Box::new(handler));
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
                        for handler in $self.handlers.get(&keys::Keys::[<$x:camel>]).unwrap_or(&vec![]) {
                            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                handler(&$event);
                            }));
                            if let Err(e) = result {
                                ($self.on_handler_panic)(e);
                            }
                        }
                    }
                )*
            }
        }
    }
}

impl TraqBot {
    pub fn new<U>(bear_token: impl Into<String>, target_url: U) -> Self
    where
        U: TryInto<Url>,
        U::Error: Debug,
    {
        let mut bot: Self = Default::default();

        let bear_token = bear_token.into();
        let mut target_url: Url = target_url.try_into().unwrap();
        let ws_target_url = match target_url.scheme() {
            "wss" | "ws" => target_url,
            "http" => {
                target_url.set_scheme("ws").unwrap();
                target_url
            }
            "https" => {
                target_url.set_scheme("wss").unwrap();
                target_url
            }
            _ => panic!(
                "unsupported scheme: {} (supported: ws, wss, http, https)",
                target_url.scheme()
            ),
        };
        let ws_origin = ws_target_url
            .origin()
            .ascii_serialization()
            .parse()
            .unwrap();
        let gateway_path = ws_target_url.path().to_owned();

        bot.bear_token = bear_token;
        bot.ws_origin = ws_origin;
        bot.gateway_path = gateway_path;

        bot
    }

    /// BOT を起動する
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
            .header("Authorization", format!("Bearer {}", self.bear_token))
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
    pub fn get_ws_origin(&self) -> Url {
        self.ws_origin.clone()
    }
    /// http もしくは https で始まる origin に相当する URL を返す
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
    pub fn get_ws_url(&self) -> Url {
        self.ws_origin.join(&self.gateway_path).unwrap()
    }
    /// http もしくは https で始まる gateway の URL を返す
    pub fn get_http_url(&self) -> Url {
        let mut url = self.get_ws_url();
        match url.scheme() {
            "wss" => url.set_scheme("https").unwrap(),
            "ws" => url.set_scheme("http").unwrap(),
            _ => panic!("Invalid scheme: {} (expected: ws, wss)", url.scheme()),
        }
        url
    }

    /// bot access token を更新する
    pub fn set_bear_token(&mut self, bear_token: &str) {
        self.bear_token = bear_token.into();
    }
    /// bot access token を返す
    pub fn get_bear_token(&self) -> &str {
        &self.bear_token
    }

    /// 登録したハンドラが panic した際のハンドラを設定する
    ///
    /// **Warning**: このハンドラが panic した場合、プログラムが終了します
    pub fn set_on_panic_handler<F: Fn(OnPanic) + 'static>(&mut self, handler: F) {
        self.on_handler_panic = Box::new(handler);
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

    /// key のイベントに対応するハンドラを設定する
    ///
    /// ハンドラに渡される enum は key で指定したイベントであることが保証される
    pub fn on_event<F: Fn(&Events) + 'static>(mut self, key: keys::Keys, handler: F) -> Self {
        self.handlers
            .entry(key)
            .or_insert(vec![])
            .push(Box::new(handler));
        self
    }
    /// key のイベントに対応するハンドラを複数同時に設定する
    ///
    /// ハンドラに渡される enum は key で指定したイベントであることが保証される
    pub fn on_event_multi<F: Fn(&Events) + 'static, Fs: Into<Vec<F>>>(
        mut self,
        key: keys::Keys,
        handlers: Fs,
    ) -> Self {
        let handlers: Vec<_> = handlers.into();
        let entry = self.handlers.entry(key).or_insert(vec![]);
        for handler in handlers {
            entry.push(Box::new(handler));
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
    pub fn on_error<F: Fn(&str) + 'static>(mut self, handler: F) -> Self {
        let handler = convert_handler!(handler => Error);
        self.handlers
            .entry(keys::Keys::Error)
            .or_insert(vec![])
            .push(Box::new(handler));
        self
    }
    #[doc = "Error イベントを受け取った際のハンドラを複数同時に登録する"]
    pub fn on_error_multi<F: Fn(&str) + 'static, Fs: Into<Vec<F>>>(mut self, handlers: Fs) -> Self {
        let handlers: Vec<_> = handlers.into();
        let entry = self.handlers.entry(keys::Keys::Error).or_insert(vec![]);
        for handler in handlers {
            let handler = convert_handler!(handler => Error);
            entry.push(Box::new(handler));
        }
        self
    }
}

impl Default for TraqBot {
    fn default() -> Self {
        Self {
            bear_token: Default::default(),
            ws_origin: Url::parse("wss://q.trap.jp").unwrap(),
            gateway_path: "/api/v3/bot/ws".to_owned(),
            handlers: Default::default(),
            on_handler_panic: Box::new(|e| {
                eprintln!("{:?}", e);
            }),
        }
    }
}
