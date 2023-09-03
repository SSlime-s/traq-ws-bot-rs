use serde::{Deserialize, Serialize};

use super::macros::gp_struct;
use super::{Time, Uuid};

gp_struct!(
    struct Base {
        event_time: Time,
    }

    struct User {
        id: Uuid,
        name: Uuid,
        display_name: String,
        icon_id: Uuid,
        bot: bool,
    }

    struct Channel {
        id: Uuid,
        name: String,
        path: String,
        parent_id: Uuid,
        creator: User,
        created_at: Time,
        updated_at: Time,
    }

    struct Message {
        id: Uuid,
        user: User,
        channel_id: Uuid,
        text: String,
        plain_text: String,
        embedded: Vec<EmbeddedInfo>,
        created_at: Time,
        updated_at: Time,
    }

    struct GroupMember {
        group_id: Uuid,
        user_id: Uuid,
    }

    struct GroupMemberWithRole {
        group_id: Uuid,
        user_id: Uuid,
        role: String,
    }

    struct Group {
        id: Uuid,
        name: String,
        description: String,
        #[serde(rename = "type")]
        type_: String,
        icon: Uuid,
        admins: Vec<GroupMember>,
        members: Vec<GroupMemberWithRole>,
        created_at: Time,
        updated_at: Time,
    }

    struct DeletedMessage {
        id: Uuid,
        channel_id: Uuid,
    }

    struct DeletedDirectMessage {
        id: Uuid,
        user_id: Uuid,
        channel_id: Uuid,
    }

    struct EmbeddedInfo {
        raw: String,
        #[serde(rename = "type")]
        type_: String,
        id: Uuid,
    }

    struct MessageStamp {
        stamp_id: Uuid,
        user_id: Uuid,
        count: i64,
        created_at: Time,
        updated_at: Time,
    }
);
