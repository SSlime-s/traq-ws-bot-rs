/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostStarRequest : スター追加リクエスト



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostStarRequest {
    /// チャンネルUUID
    #[serde(rename = "channelId")]
    pub channel_id: uuid::Uuid,
}

impl PostStarRequest {
    /// スター追加リクエスト
    pub fn new(channel_id: uuid::Uuid) -> PostStarRequest {
        PostStarRequest {
            channel_id,
        }
    }
}


