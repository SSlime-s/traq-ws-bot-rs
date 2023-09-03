pub mod common;
pub mod macros;
pub mod payload;

use chrono::{DateTime, FixedOffset};
use paste::paste;

use self::payload::*;
use serde::{self, Deserialize, Serialize};

pub type Uuid = String;
pub type Time = DateTime<FixedOffset>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "body")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Events {
    Ping(Ping),
    Joined(Joined),
    Left(Left),
    MessageCreated(MessageCreated),
    MessageUpdated(MessageUpdated),
    MessageDeleted(MessageDeleted),
    BotMessageStampsUpdated(BotMessageStampsUpdated),
    DirectMessageCreated(DirectMessageCreated),
    DirectMessageUpdated(DirectMessageUpdated),
    DirectMessageDeleted(DirectMessageDeleted),
    ChannelCreated(ChannelCreated),
    ChannelTopicChanged(ChannelTopicChanged),
    UserCreated(UserCreated),
    UserGroupCreated(UserGroupCreated),
    UserGroupUpdated(UserGroupUpdated),
    UserGroupDeleted(UserGroupDeleted),
    UserGroupMemberAdded(UserGroupMemberAdded),
    UserGroupMemberUpdated(UserGroupMemberUpdated),
    UserGroupMemberRemoved(UserGroupMemberRemoved),
    UserGroupAdminAdded(UserGroupAdminAdded),
    UserGroupAdminRemoved(UserGroupAdminRemoved),
    StampCreated(StampCreated),
    TagAdded(TagAdded),
    TagRemoved(TagRemoved),
    Error(String),
}

macro_rules! from_inner {
    ($($name:ident),*$(,)?) => {
        paste! {
            $(
                impl From<[<$name:camel>]> for Events {
                    fn from([<$name:snake>]: [<$name:camel>]) -> Self {
                        Events::$name([<$name:snake>])
                    }
                }
            )*
        }
    };
}

from_inner!(
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

macro_rules! try_from_to_inner {
    ($($name:ident),*$(,)?) => {
        paste! {
            $(
                impl TryFrom<Events> for [<$name:camel>] {
                    type Error = ();
                    fn try_from(event: Events) -> Result<Self, Self::Error> {
                        match event {
                            Events::[<$name:camel>]([<$name:snake>]) => Ok([<$name:snake>]),
                            _ => Err(()),
                        }
                    }
                }
                impl<'a> TryFrom<&'a Events> for &'a [<$name:camel>] {
                    type Error = ();
                    fn try_from(event: &'a Events) -> Result<Self, Self::Error> {
                        match event {
                            Events::[<$name:camel>]([<$name:snake>]) => Ok([<$name:snake>]),
                            _ => Err(()),
                        }
                    }
                }
            )*
        }
    };
}

try_from_to_inner!(
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
