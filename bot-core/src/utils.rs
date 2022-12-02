use std::sync::Arc;

use tokio::sync::{Mutex, Semaphore};

use crate::events::common::Message;
#[cfg(feature = "openapi")]
use crate::openapi;

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

/// BOT の access token から openapi の configuration を作成する
#[cfg(feature = "openapi")]
pub fn create_configuration(bot_access_token: impl Into<String>) -> openapi::apis::configuration::Configuration {
    openapi::apis::configuration::Configuration {
        bearer_access_token: Some(bot_access_token.into()),
        ..Default::default()
    }
}

/// interval 間に最大 max_count 回しか実行されないようにすることができる struct
///
/// **Default:** `interval` は 5 秒, `max_count` は 5 回
///
/// # Example
/// ```
/// use std::time::Duration;
/// use traq_ws_bot::utils::RateLimiter;
///
/// let mut limiter = RateLimiter::new(5, Duration::from_secs(5)); // RateLimiter::default() と同じ
/// loop {
///    if limiter.try_acquire() {
///        // 5秒間に5回しか実行されない
///        println!("Hello");
///    }
/// }
/// ```
pub struct RateLimiter {
    interval: std::time::Duration,
    semaphore: Arc<Semaphore>,
}
impl RateLimiter {
    /// interval 間に最大 max_count 回しか実行されないようにすることができる struct を作成する
    pub fn new(max_count: usize, interval: std::time::Duration) -> Self {
        Self {
            interval,
            semaphore: Arc::new(Semaphore::new(max_count)),
        }
    }

    /// lock を取得するまで待機する
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use traq_ws_bot::utils::RateLimiter;
    ///
    /// let mut limiter = RateLimiter::new(5, Duration::from_secs(5)); // RateLimiter::default() と同じ
    /// loop {
    ///     await limiter.acquire();
    ///     // 5秒間に5回しか実行されない
    ///     println!("Hello");
    /// }
    pub async fn acquire(&self) -> () {
        let semaphore = self.semaphore.clone();
        let permit = semaphore.acquire_owned().await.unwrap();
        let interval = self.interval;

        tokio::spawn( async move {
            tokio::time::sleep(interval).await;
            drop(permit);
        });
    }

    /// lock を取得できたら true を返す
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    /// use traq_ws_bot::utils::RateLimiter;
    ///
    /// let mut limiter = RateLimiter::new(5, Duration::from_secs(5)); // RateLimiter::default() と同じ
    /// loop {
    ///     if limiter.try_acquire() {
    ///        // 5秒間に5回しか実行されない
    ///       println!("Hello");
    ///    }
    /// }
    pub fn try_acquire(&self) -> bool {
        let semaphore = self.semaphore.clone();
        match semaphore.try_acquire_owned() {
            Ok(permit) => {
                let interval = self.interval;
                tokio::spawn( async move {
                    tokio::time::sleep(interval).await;
                    drop(permit);
                });
                true
            }
            Err(_) => false,
        }
    }
}
impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(5, std::time::Duration::from_secs(5))
    }
}
