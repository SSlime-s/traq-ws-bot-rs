/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebRtcAuthenticateResult : skyway用認証リクエストリザルト



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebRtcAuthenticateResult {
    /// ピアID
    #[serde(rename = "peerId")]
    pub peer_id: String,
    /// TTL
    #[serde(rename = "ttl")]
    pub ttl: i32,
    /// タイムスタンプ
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// 認証トークン
    #[serde(rename = "authToken")]
    pub auth_token: String,
}

impl WebRtcAuthenticateResult {
    /// skyway用認証リクエストリザルト
    pub fn new(peer_id: String, ttl: i32, timestamp: i64, auth_token: String) -> WebRtcAuthenticateResult {
        WebRtcAuthenticateResult {
            peer_id,
            ttl,
            timestamp,
            auth_token,
        }
    }
}


