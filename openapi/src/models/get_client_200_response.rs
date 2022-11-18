/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetClient200Response {
    /// クライアントUUID
    #[serde(rename = "id")]
    pub id: String,
    /// クライアント名
    #[serde(rename = "name")]
    pub name: String,
    /// 説明
    #[serde(rename = "description")]
    pub description: String,
    /// クライアント開発者UUID
    #[serde(rename = "developerId")]
    pub developer_id: uuid::Uuid,
    /// 要求スコープの配列
    #[serde(rename = "scopes")]
    pub scopes: Vec<crate::models::OAuth2Scope>,
    /// コールバックURL
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// クライアントシークレット
    #[serde(rename = "secret")]
    pub secret: String,
}

impl GetClient200Response {
    pub fn new(id: String, name: String, description: String, developer_id: uuid::Uuid, scopes: Vec<crate::models::OAuth2Scope>, callback_url: String, secret: String) -> GetClient200Response {
        GetClient200Response {
            id,
            name,
            description,
            developer_id,
            scopes,
            callback_url,
            secret,
        }
    }
}


