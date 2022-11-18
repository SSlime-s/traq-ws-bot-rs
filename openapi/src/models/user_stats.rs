/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserStats : ユーザー統計情報



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserStats {
    /// ユーザーの総投稿メッセージ数(削除されたものも含む)
    #[serde(rename = "totalMessageCount")]
    pub total_message_count: i64,
    /// ユーザーのスタンプ統計情報
    #[serde(rename = "stamps")]
    pub stamps: Vec<crate::models::UserStatsStamp>,
    /// 統計情報日時
    #[serde(rename = "datetime")]
    pub datetime: String,
}

impl UserStats {
    /// ユーザー統計情報
    pub fn new(total_message_count: i64, stamps: Vec<crate::models::UserStatsStamp>, datetime: String) -> UserStats {
        UserStats {
            total_message_count,
            stamps,
            datetime,
        }
    }
}

