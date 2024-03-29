/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchUserTagRequest : ユーザーのタグの編集リクエスト



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PatchUserTagRequest {
    /// タグのロック状態
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
}

impl PatchUserTagRequest {
    /// ユーザーのタグの編集リクエスト
    pub fn new(is_locked: bool) -> PatchUserTagRequest {
        PatchUserTagRequest {
            is_locked,
        }
    }
}


