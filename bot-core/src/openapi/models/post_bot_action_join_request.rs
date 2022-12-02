/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostBotActionJoinRequest : BOTチャンネル参加リクエスト



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostBotActionJoinRequest {
    /// チャンネルUUID
    #[serde(rename = "channelId")]
    pub channel_id: uuid::Uuid,
}

impl PostBotActionJoinRequest {
    /// BOTチャンネル参加リクエスト
    pub fn new(channel_id: uuid::Uuid) -> PostBotActionJoinRequest {
        PostBotActionJoinRequest {
            channel_id,
        }
    }
}


