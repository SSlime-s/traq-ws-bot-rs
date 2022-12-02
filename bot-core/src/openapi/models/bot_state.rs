/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BotState : BOT状態 0: 停止 1: 有効 2: 一時停止

/// BOT状態 0: 停止 1: 有効 2: 一時停止
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BotState {
    #[serde(rename = "0")]
    Deactivated,
    #[serde(rename = "1")]
    Active,
    #[serde(rename = "2")]
    Suspended,

}

impl ToString for BotState {
    fn to_string(&self) -> String {
        match self {
            Self::Deactivated => String::from("0"),
            Self::Active => String::from("1"),
            Self::Suspended => String::from("2"),
        }
    }
}

impl Default for BotState {
    fn default() -> BotState {
        Self::Deactivated
    }
}




