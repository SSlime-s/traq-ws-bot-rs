use std::{collections::HashSet, sync::Arc, time::Duration};

use futures::{
    future::{self, BoxFuture},
    pin_mut, Future, StreamExt,
};
use paste::paste;
use reqwest::Url;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{handshake::client::generate_key, Message},
};

use crate::events::{payload, Events};

pub mod handler;
pub mod keys;

use self::handler::Handler;

pub const TRAQ_ORIGIN: &str = "https://q.trap.jp";
pub const TRAQ_ORIGIN_WS: &str = "wss://q.trap.jp";

pub const TRAQ_WS_GATEWAY_PATH: &str = "/api/v3/bots/ws";

pub const INITIAL_RETRY_WAIT: Duration = Duration::from_secs(3);
pub const MAX_RETRY_WAIT: Duration = Duration::from_secs(10 * 60);

pub struct TraqBotBuilder<T: Send + Sync + 'static> {
    authorization_scheme: String,
    token: String,
    target_url: Url,
    handlers: [Vec<Arc<dyn Handler<T>>>; keys::KEYS_COUNT],
    resource: Option<T>,
}

pub struct TraqBot<T: Send + Sync + 'static> {
    authorization_scheme: String,
    token: String,
    ws_origin: Url,
    gateway_path: String,
    handlers: [Box<[Arc<dyn Handler<T>>]>; keys::KEYS_COUNT],
    resource: Arc<T>,
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
                #[doc = "    ."[<on_ $x:snake>]"(|event| async move {"]
                #[doc = "        println!(\"{:?}\", event);"]
                #[doc = "    })"]
                #[doc = "    .build();"]
                #[doc = "```"]
                pub fn [<on_ $x:snake>]<Fut>(mut self, handler: fn(payload::[<$x:camel>]) -> Fut) -> Self
                where
                    Fut: Future<Output = ()> + std::marker::Send + 'static,
                {
                    self.handlers[keys::Keys::[<$x:camel>] as usize].push(Arc::new(handler));
                    self
                }
                #[doc = ""[<$x:camel>]" イベントを受け取った際のハンドラを登録する"]
                #[doc = "引数から resource を取得することができる"]
                #[doc = ""]
                #[doc = "# Example"]
                #[doc = "```rust"]
                #[doc = "use traq_ws_bot::bot::builder;"]
                #[doc = ""]
                #[doc = "let bot = builder(\"BOT_ACCESS_TOKEN\")"]
                #[doc = "    ."[<on_ $x:snake _with_resource>]"(|event, resource| async move {"]
                #[doc = "        println!(\"{:?}, {:?}\", event, resource);"]
                #[doc = "    })"]
                #[doc = "    .build();"]
                #[doc = "```"]
                pub fn [<on_ $x:snake _with_resource>]<Fut>(mut self, handler: fn(payload::[<$x:camel>], Arc<T>) -> Fut) -> Self
                where
                    Fut: Future<Output = ()> + std::marker::Send + 'static,
                {
                    self.handlers[keys::Keys::[<$x:camel>] as usize].push(Arc::new(handler));
                    self
                }
            }
        )*
    };
}

macro_rules! handle_event_inner {
    ($self:expr, $event:expr => {$($x:ident),*$(,)?}, $resource:expr) => {
        paste!{
            match $event {
                $(
                    Events::[<$x:camel>](_) => Box::pin(async {
                        future::join_all($self.handlers[keys::Keys::[<$x:camel>] as usize].iter().map(
                            |handler| async {
                                handler.handle($event.clone(), $resource.clone()).await;
                            },
                        ))
                        .await;
                    }),
                )*
            }
        }
    }
}

impl<T: Send + Sync + 'static> TraqBot<T> {
    /// BOT を起動する
    ///
    /// # Examples
    /// ```rust
    /// use traq_ws_bot::bot::builder;
    ///
    /// # async fn try_main() -> anyhow::Result<()> {
    /// let bot = builder("BOT_ACCESS_TOKEN")
    ///     .on_message_created(|event| async move {
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
        let mut retry_wait = INITIAL_RETRY_WAIT;

        loop {
            match self.start_inner(&host).await {
                Ok(()) => {
                    retry_wait = INITIAL_RETRY_WAIT;
                }
                Err(e) => {
                    log::error!("Error: {}", e);
                    retry_wait = (retry_wait * 2).min(MAX_RETRY_WAIT);
                }
            }

            log::info!("Disconnected. retry after {} seconds", retry_wait.as_secs());
            tokio::time::sleep(retry_wait).await;
        }
    }

    async fn start_inner(&self, host: &str) -> anyhow::Result<()> {
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
            futures::TryStreamExt::try_for_each(
                read.map(|msg| -> Result<_, ()> { Ok(msg) }),
                |message| async {
                    match message {
                        Ok(message) => match message {
                            Message::Ping(_) => {
                                // nop
                                Ok(())
                            }
                            Message::Text(content) => {
                                let event = serde_json::from_str(&content);
                                if let Ok(event) = event {
                                    self.handle_event(&event, self.resource.clone()).await;
                                } else {
                                    eprintln!("failed to parse event: {}", content);
                                }
                                Ok(())
                            }
                            Message::Close(_) => Err(()),
                            _ => {
                                eprintln!("not supported message: {:?}", message);
                                Ok(())
                            }
                        },
                        Err(e) => {
                            eprintln!("error: {:?}", e);
                            Ok(())
                        }
                    }
                },
            )
        };

        pin_mut!(write_loop, read_loop);
        future::select(read_loop, write_loop).await;

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
    async fn handle_event(&self, event: &Events, resource: Arc<T>) {
        let promise: BoxFuture<()> = handle_event_inner!(
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
                UserGroupCreated,
                UserGroupUpdated,
                UserGroupDeleted,
                UserGroupMemberAdded,
                UserGroupMemberUpdated,
                UserGroupMemberRemoved,
                UserGroupAdminAdded,
                UserGroupAdminRemoved,
                StampCreated,
                TagAdded,
                TagRemoved,
                Error,
            },
            resource
        );
        promise.await;
    }
}

/// TraqBot の Builder を作成する
pub fn builder(token: impl Into<String>) -> TraqBotBuilder<()> {
    TraqBotBuilder {
        token: token.into(),
        resource: Some(()),
        ..Default::default()
    }
}

#[doc(hidden)]
#[allow(unused)]
#[rustfmt::skip]
/*pub */ fn builder_with_config(_config: ()) -> TraqBotBuilder<()> {
    unimplemented!()
}

impl<T: Send + Sync + 'static> Default for TraqBotBuilder<T> {
    fn default() -> Self {
        let handlers_arr: [Vec<_>; keys::KEYS_COUNT] = Default::default();

        Self {
            authorization_scheme: "Bearer".to_owned(),
            token: Default::default(),
            target_url: Url::parse(TRAQ_ORIGIN_WS)
                .unwrap()
                .join(TRAQ_WS_GATEWAY_PATH)
                .unwrap(),
            handlers: handlers_arr,
            resource: Default::default(),
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

impl<T: Send + Sync + 'static> TraqBotBuilder<T> {
    /// TraqBotBuilder から TraqBot を作成する
    ///
    /// # Example
    /// ```
    /// use traq_ws_bot::bot::builder;
    ///
    /// let bot = builder("BOT_ACCESS_TOKEN")
    ///     .on_message_created(|event| async move {
    ///         println!("{:?}", event);
    ///     })
    ///    .build();
    /// ```
    pub fn build(self) -> TraqBot<T> {
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
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|v: Vec<Box<[Arc<dyn Handler<T>>]>>| {
                    format!(
                        "Invalid handlers length: {} (expected: {})",
                        v.len(),
                        keys::KEYS_COUNT
                    )
                })
                .unwrap(),
            resource: Arc::new(self.resource.unwrap()),
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
    pub fn set_target_url<U>(mut self, url: U) -> Self
    where
        U: TryInto<Url>,
        U::Error: std::fmt::Debug,
    {
        self.target_url = url.try_into().unwrap();
        self
    }

    /// keys のイベントに対応するハンドラを設定する
    /// keys は複数同時に指定可能
    ///
    /// ハンドラに渡される enum は key で指定したイベントに含まれることが保証される
    ///
    /// **NOTE** 全ての key を指定したい場合は KEYS_ALL を用いると良い
    ///
    /// # Example
    /// ```rust
    /// use traq_ws_bot::{bot::{builder, keys::Keys}, events::Events};
    ///
    /// let bot = builder("BOT_ACCESS_TOKEN")
    ///     .on_event(Keys::Joined, |event| async move {
    ///        if let Events::Joined(event) = event {
    ///           println!("{:?}", event);
    ///       }
    ///    })
    ///   .build();
    /// ```
    ///
    /// ```rust
    /// use traq_ws_bot::{bot::{builder, keys::Keys}, events::Events};
    ///
    /// let bot = builder("BOT_ACCESS_TOKEN")
    ///     .on_event([Keys::Joined, Keys::Left], |event| async move {
    ///         match event {
    ///             Events::Joined(event) => println!("{:?}", event),
    ///             Events::Left(event) => println!("{:?}", event),
    ///             _ => unreachable!(),
    ///         }
    ///     })
    ///     .build();
    /// ```
    pub fn on_event<Fut, K>(mut self, keys: K, handler: fn(Events) -> Fut) -> Self
    where
        Fut: Future<Output = ()> + std::marker::Send + 'static,
        K: IntoIterator<Item = keys::Keys>,
    {
        let keys_set = keys.into_iter().collect::<HashSet<_>>();
        let handler = Arc::new(handler);
        for key in keys_set {
            self.handlers[key as usize].push(handler.clone());
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
        UserGroupCreated,
        UserGroupUpdated,
        UserGroupDeleted,
        UserGroupMemberAdded,
        UserGroupMemberUpdated,
        UserGroupMemberRemoved,
        UserGroupAdminAdded,
        UserGroupAdminRemoved,
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
    #[doc = "    .on_error(|event| async move {"]
    #[doc = "        println!(\"{:?}\", event);"]
    #[doc = "    })"]
    #[doc = "    .build();"]
    #[doc = "```"]
    pub fn on_error<Fut>(mut self, handler: fn(String) -> Fut) -> Self
    where
        Fut: Future<Output = ()> + std::marker::Send + 'static,
    {
        self.handlers[keys::Keys::Error as usize].push(Arc::new(handler));
        self
    }
    #[doc = "Error イベントを受け取った際のハンドラを登録する"]
    #[doc = "引数から resource を取得することができる"]
    #[doc = ""]
    #[doc = "# Example"]
    #[doc = "```rust"]
    #[doc = "use traq_ws_bot::bot::builder;"]
    #[doc = ""]
    #[doc = "let bot = builder(\"BOT_ACCESS_TOKEN\")"]
    #[doc = "    .on_error_with_resource(|event, resource| async move {"]
    #[doc = "        println!(\"{:?}, {:?}\", event, resource);"]
    #[doc = "    })"]
    #[doc = "    .build();"]
    #[doc = "```"]
    pub fn on_error_with_resource<Fut>(mut self, handler: fn(String, Arc<T>) -> Fut) -> Self
    where
        Fut: Future<Output = ()> + std::marker::Send + 'static,
    {
        self.handlers[keys::Keys::Error as usize].push(Arc::new(handler));
        self
    }

    #[doc = "Resource を登録する"]
    #[doc = ""]
    #[doc = "**Warning**: これより前に登録したハンドラは削除される"]
    #[doc = ""]
    #[doc = "# Example"]
    #[doc = "```rust"]
    #[doc = "use traq_ws_bot::bot::builder;"]
    #[doc = ""]
    #[doc = "let bot = builder(\"BOT_ACCESS_TOKEN\")"]
    #[doc = "    .insert_resource(\"Hello, world!\")"]
    #[doc = "    .build();"]
    #[doc = "```"]
    pub fn insert_resource<U>(self, resource: U) -> TraqBotBuilder<U>
    where
        U: Send + Sync + 'static,
    {
        TraqBotBuilder {
            token: self.token,
            target_url: self.target_url,
            resource: Some(resource),
            authorization_scheme: self.authorization_scheme,
            ..Default::default()
        }
    }
}

async fn tmp(_: String) {
    println!("tmp");
}

#[allow(dead_code)]
async fn tmp2() {
    let _bot = builder("")
        .on_error(tmp)
        .on_error(|_: String| async move { println!("tmp") })
        .on_event(keys::Keys::Error, |event| async {
            if let Events::Error(event) = event {
                println!("{:?}", event);
                tmp(event).await;
            }
        });
}
