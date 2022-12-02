/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StampStats : スタンプ統計情報



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StampStats {
    /// スタンプ使用総数(同じユーザによって同じメッセージに貼られたものは複数カウントしない)
    #[serde(rename = "count")]
    pub count: i64,
    /// スタンプ使用総数(全てカウント)
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}

impl StampStats {
    /// スタンプ統計情報
    pub fn new(count: i64, total_count: i64) -> StampStats {
        StampStats {
            count,
            total_count,
        }
    }
}


