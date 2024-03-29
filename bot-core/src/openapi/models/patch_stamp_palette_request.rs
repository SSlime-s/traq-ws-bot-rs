/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchStampPaletteRequest : スタンプパレット情報変更リクエスト



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PatchStampPaletteRequest {
    /// パレット名
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 説明
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// パレット内のスタンプUUIDの配列
    #[serde(rename = "stamps", skip_serializing_if = "Option::is_none")]
    pub stamps: Option<Vec<uuid::Uuid>>,
}

impl PatchStampPaletteRequest {
    /// スタンプパレット情報変更リクエスト
    pub fn new() -> PatchStampPaletteRequest {
        PatchStampPaletteRequest {
            name: None,
            description: None,
            stamps: None,
        }
    }
}


