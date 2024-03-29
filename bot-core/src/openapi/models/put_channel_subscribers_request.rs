/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PutChannelSubscribersRequest : 通知をオンにするユーザーのUUID配列



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutChannelSubscribersRequest {
    /// 通知をオンにするユーザーのUUID配列
    #[serde(rename = "on")]
    pub on: Vec<uuid::Uuid>,
}

impl PutChannelSubscribersRequest {
    /// 通知をオンにするユーザーのUUID配列
    pub fn new(on: Vec<uuid::Uuid>) -> PutChannelSubscribersRequest {
        PutChannelSubscribersRequest {
            on,
        }
    }
}


