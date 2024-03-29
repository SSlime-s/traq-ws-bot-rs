/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DmChannel : ダイレクトメッセージチャンネル



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DmChannel {
    /// チャンネルUUID
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// 送信先相手のUUID
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
}

impl DmChannel {
    /// ダイレクトメッセージチャンネル
    pub fn new(id: uuid::Uuid, user_id: uuid::Uuid) -> DmChannel {
        DmChannel {
            id,
            user_id,
        }
    }
}


