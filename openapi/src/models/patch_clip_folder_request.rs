/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchClipFolderRequest : クリップフォルダ情報編集リクエスト



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PatchClipFolderRequest {
    /// フォルダ名
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 説明
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PatchClipFolderRequest {
    /// クリップフォルダ情報編集リクエスト
    pub fn new() -> PatchClipFolderRequest {
        PatchClipFolderRequest {
            name: None,
            description: None,
        }
    }
}


