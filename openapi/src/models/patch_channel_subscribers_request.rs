/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchChannelSubscribersRequest : チャンネル購読者編集リクエスト



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PatchChannelSubscribersRequest {
    /// 通知をオンにするユーザーのUUID配列
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    pub on: Option<Vec<uuid::Uuid>>,
    /// 通知をオフにするユーザーのUUID配列
    #[serde(rename = "off", skip_serializing_if = "Option::is_none")]
    pub off: Option<Vec<uuid::Uuid>>,
}

impl PatchChannelSubscribersRequest {
    /// チャンネル購読者編集リクエスト
    pub fn new() -> PatchChannelSubscribersRequest {
        PatchChannelSubscribersRequest {
            on: None,
            off: None,
        }
    }
}


