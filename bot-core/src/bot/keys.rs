#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keys {
    Ping = 0,
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

pub const KEYS_COUNT: usize = Keys::Error as usize + 1;
