use std::sync::Arc;

use async_trait::async_trait;
use futures::Future;
use paste::paste;

use crate::events::{payload, Events};

#[async_trait]
pub trait Handler<T: Send + Sync + 'static>: Send + Sync + 'static {
    async fn handle(&self, event: Events, resource: Arc<T>) -> ();
}

#[async_trait]
impl<T: Send + Sync + 'static, Fut> Handler<T> for fn(Events) -> Fut
where
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn handle(&self, event: Events, _: Arc<T>) {
        (self)(event).await
    }
}
#[async_trait]
impl<T: Send + Sync + 'static, Fut> Handler<T> for fn(Events, Arc<T>) -> Fut
where
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn handle(&self, event: Events, resource: Arc<T>) {
        (self)(event, resource).await
    }
}

#[async_trait]
impl<T: Send + Sync + 'static, Fut> Handler<T> for fn(String) -> Fut
where
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn handle(&self, event: Events, _: Arc<T>) {
        if let Events::Error(payload) = event {
            self(payload).await
        }
    }
}
#[async_trait]
impl<T: Send + Sync + 'static, Fut> Handler<T> for fn(String, Arc<T>) -> Fut
where
    Fut: Future<Output = ()> + Send + 'static,
{
    async fn handle(&self, event: Events, resource: Arc<T>) {
        if let Events::Error(payload) = event {
            self(payload, resource).await
        }
    }
}

macro_rules! x_handler_structs {
    ($($x:ident),*$(,)?) => {
        $(
            paste! {
                #[async_trait]
                impl<T: Send + Sync + 'static, Fut> Handler<T> for fn(payload::[<$x:camel>]) -> Fut
                where
                    Fut: Future<Output = ()> + Send + 'static,
                {
                    async fn handle(&self, event: Events, _: Arc<T>) {
                        if let Events::[<$x:camel>](payload) = event {
                            self(payload).await
                        }
                    }
                }
                #[async_trait]
                impl<T: Send + Sync + 'static, Fut> Handler<T> for fn(payload::[<$x:camel>], Arc<T>) -> Fut
                where
                    Fut: Future<Output = ()> + Send + 'static,
                {
                    async fn handle(&self, event: Events, resource: Arc<T>) {
                        if let Events::[<$x:camel>](payload) = event {
                            self(payload, resource).await
                        }
                    }
                }
            }
        )*
    };
}

x_handler_structs!(
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
