/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PutChannelTopicRequest : チャンネルトピック編集リクエスト



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutChannelTopicRequest {
    /// トピック
    #[serde(rename = "topic")]
    pub topic: String,
}

impl PutChannelTopicRequest {
    /// チャンネルトピック編集リクエスト
    pub fn new(topic: String) -> PutChannelTopicRequest {
        PutChannelTopicRequest {
            topic,
        }
    }
}

