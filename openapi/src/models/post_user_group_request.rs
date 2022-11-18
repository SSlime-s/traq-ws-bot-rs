/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostUserGroupRequest : ユーザーグループ作成リクエスト



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostUserGroupRequest {
    /// グループ名
    #[serde(rename = "name")]
    pub name: String,
    /// 説明
    #[serde(rename = "description")]
    pub description: String,
    /// グループタイプ
    #[serde(rename = "type")]
    pub r#type: String,
}

impl PostUserGroupRequest {
    /// ユーザーグループ作成リクエスト
    pub fn new(name: String, description: String, r#type: String) -> PostUserGroupRequest {
        PostUserGroupRequest {
            name,
            description,
            r#type,
        }
    }
}


