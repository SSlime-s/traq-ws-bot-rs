/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Tag : タグ情報



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Tag {
    /// タグUUID
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// タグ文字列
    #[serde(rename = "tag")]
    pub tag: String,
    /// タグがつけられているユーザーのUUID配列
    #[serde(rename = "users")]
    pub users: Vec<uuid::Uuid>,
}

impl Tag {
    /// タグ情報
    pub fn new(id: uuid::Uuid, tag: String, users: Vec<uuid::Uuid>) -> Tag {
        Tag {
            id,
            tag,
            users,
        }
    }
}


