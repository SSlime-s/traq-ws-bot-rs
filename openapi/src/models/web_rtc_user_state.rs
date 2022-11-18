/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebRtcUserState : WebRTC状態



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebRtcUserState {
    /// ユーザーUUID
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
    /// チャンネルUUID
    #[serde(rename = "channelId")]
    pub channel_id: uuid::Uuid,
    /// セッションの配列
    #[serde(rename = "sessions")]
    pub sessions: Vec<crate::models::WebRtcUserStateSessionsInner>,
}

impl WebRtcUserState {
    /// WebRTC状態
    pub fn new(user_id: uuid::Uuid, channel_id: uuid::Uuid, sessions: Vec<crate::models::WebRtcUserStateSessionsInner>) -> WebRtcUserState {
        WebRtcUserState {
            user_id,
            channel_id,
            sessions,
        }
    }
}


