/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ClippedMessage : クリップされたメッセージ



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClippedMessage {
    #[serde(rename = "message")]
    pub message: Box<crate::models::Message>,
    /// クリップした日時
    #[serde(rename = "clippedAt")]
    pub clipped_at: String,
}

impl ClippedMessage {
    /// クリップされたメッセージ
    pub fn new(message: crate::models::Message, clipped_at: String) -> ClippedMessage {
        ClippedMessage {
            message: Box::new(message),
            clipped_at,
        }
    }
}

