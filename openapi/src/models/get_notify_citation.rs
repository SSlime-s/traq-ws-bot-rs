/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetNotifyCitation : メッセージ引用通知の設定情報



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetNotifyCitation {
    #[serde(rename = "notifyCitation")]
    pub notify_citation: bool,
}

impl GetNotifyCitation {
    /// メッセージ引用通知の設定情報
    pub fn new(notify_citation: bool) -> GetNotifyCitation {
        GetNotifyCitation {
            notify_citation,
        }
    }
}


