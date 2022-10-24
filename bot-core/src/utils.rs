use crate::events::common::Message;

/// メッセージに `target_user_id` に対するメンションが含まれているかどうかを返す
pub fn is_mentioned_message(message: &Message, target_user_id: &str) -> bool {
    message
        .embedded
        .iter()
        .any(|emb| emb.id.as_str() == target_user_id)
}

/// BOT の access token から簡単な API Client を作成する
pub fn create_client(bot_access_token: impl Into<String>) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    let authorization_token = format!("Bearer {}", bot_access_token.into());
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&authorization_token).unwrap(),
    );
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}
