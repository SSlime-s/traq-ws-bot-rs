use super::{common::*, macros::gp_struct, Uuid};
use serde::{Deserialize, Serialize};

gp_struct!(
    struct Ping {
        #[serde(flatten)]
        base: Base,
    }

    struct Joined {
        #[serde(flatten)]
        base: Base,

        channel: Channel,
    }

    struct Left {
        #[serde(flatten)]
        base: Base,

        channel: Channel,
    }

    struct MessageCreated {
        #[serde(flatten)]
        base: Base,

        message: Message,
    }

    struct MessageUpdated {
        #[serde(flatten)]
        base: Base,

        message: Message,
    }

    struct MessageDeleted {
        #[serde(flatten)]
        base: Base,

        message: DeletedMessage,
    }

    struct BotMessageStampsUpdated {
        #[serde(flatten)]
        base: Base,

        message_id: Uuid,
        stamps: Vec<MessageStamp>,
    }

    struct DirectMessageCreated {
        #[serde(flatten)]
        base: Base,

        message: Message,
    }

    struct DirectMessageUpdated {
        #[serde(flatten)]
        base: Base,

        message: Message,
    }

    struct DirectMessageDeleted {
        #[serde(flatten)]
        base: Base,

        message: DeletedDirectMessage,
    }

    struct ChannelCreated {
        #[serde(flatten)]
        base: Base,

        channel: Channel,
    }

    struct ChannelTopicChanged {
        #[serde(flatten)]
        base: Base,

        channel: Channel,
        topic: String,
        updater: User,
    }

    struct UserCreated {
        #[serde(flatten)]
        base: Base,

        user: User,
    }

    struct StampCreated {
        #[serde(flatten)]
        base: Base,

        id: Uuid,
        name: String,
        file_id: Uuid,
        creator: User,
    }

    struct TagAdded {
        #[serde(flatten)]
        base: Base,

        tag_id: Uuid,
        tag: String,
    }

    struct TagRemoved {
        #[serde(flatten)]
        base: Base,

        tag_id: Uuid,
        tag: String,
    }
);
