/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Ogp : OGPの情報

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Ogp {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "images")]
    pub images: Vec<crate::openapi::models::OgpMedia>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "videos")]
    pub videos: Vec<crate::openapi::models::OgpMedia>,
}

impl Ogp {
    /// OGPの情報
    pub fn new(
        r#type: String,
        title: String,
        url: String,
        images: Vec<crate::openapi::models::OgpMedia>,
        description: String,
        videos: Vec<crate::openapi::models::OgpMedia>,
    ) -> Ogp {
        Ogp {
            r#type,
            title,
            url,
            images,
            description,
            videos,
        }
    }
}