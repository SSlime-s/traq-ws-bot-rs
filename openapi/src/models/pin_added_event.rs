/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PinAddedEvent : ピン追加イベント



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PinAddedEvent {
    /// 変更者UUID
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
    /// メッセージUUID
    #[serde(rename = "messageId")]
    pub message_id: uuid::Uuid,
}

impl PinAddedEvent {
    /// ピン追加イベント
    pub fn new(user_id: uuid::Uuid, message_id: uuid::Uuid) -> PinAddedEvent {
        PinAddedEvent {
            user_id,
            message_id,
        }
    }
}


