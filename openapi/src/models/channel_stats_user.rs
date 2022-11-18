/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChannelStatsUser : チャンネル上の特定ユーザー統計情報



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelStatsUser {
    /// ユーザーID
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// メッセージ数
    #[serde(rename = "messageCount")]
    pub message_count: i64,
}

impl ChannelStatsUser {
    /// チャンネル上の特定ユーザー統計情報
    pub fn new(id: uuid::Uuid, message_count: i64) -> ChannelStatsUser {
        ChannelStatsUser {
            id,
            message_count,
        }
    }
}


