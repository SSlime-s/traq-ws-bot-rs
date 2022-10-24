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

impl IntoIterator for Keys {
    type Item = Keys;
    type IntoIter = std::array::IntoIter<Keys, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self].into_iter()
    }
}

pub const KEYS_ALL: [Keys; 17] = [
    Keys::Ping,
    Keys::Joined,
    Keys::Left,
    Keys::MessageCreated,
    Keys::MessageUpdated,
    Keys::MessageDeleted,
    Keys::BotMessageStampsUpdated,
    Keys::DirectMessageCreated,
    Keys::DirectMessageUpdated,
    Keys::DirectMessageDeleted,
    Keys::ChannelCreated,
    Keys::ChannelTopicChanged,
    Keys::UserCreated,
    Keys::StampCreated,
    Keys::TagAdded,
    Keys::TagRemoved,
    Keys::Error,
];

pub const KEYS_COUNT: usize = Keys::Error as usize + 1;
