/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FileInfoThumbnail : サムネイル情報 サムネイルが存在しない場合はnullになります Deprecated: thumbnailsを参照してください



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FileInfoThumbnail {
    /// MIMEタイプ
    #[serde(rename = "mime")]
    pub mime: String,
    /// サムネイル幅
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// サムネイル高さ
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

impl FileInfoThumbnail {
    /// サムネイル情報 サムネイルが存在しない場合はnullになります Deprecated: thumbnailsを参照してください
    pub fn new(mime: String) -> FileInfoThumbnail {
        FileInfoThumbnail {
            mime,
            width: None,
            height: None,
        }
    }
}


