use traq_ws_bot::{
    bot::{builder, TRAQ_ORIGIN},
    utils::{create_client, RateLimiter},
};

struct Resource {
    bot_access_token: String,
    limiter: RateLimiter,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let bot_access_token = std::env::var("BOT_ACCESS_TOKEN").unwrap();
    let limiter = RateLimiter::default();
    let resource = Resource {
        bot_access_token: bot_access_token.clone(),
        limiter,
    };

    let bot = builder(&bot_access_token)
        .insert_resource(resource)
        .on_direct_message_created_with_resource(|event, resource| async move {
            let client = create_client(resource.bot_access_token.as_str());
            let url = format!(
                "{}/api/v3/channels/{}/messages",
                TRAQ_ORIGIN, event.message.channel_id
            );

            let body = serde_json::json!({
                "content": event.message.plain_text,
                "embed": false
            });

            if !resource.limiter.try_acquire().await {
                dbg!("Rate limit exceeded");
                return;
            }

            let res = client.post(&url).json(&body).send().await.unwrap();
            let json = res.json::<serde_json::Value>().await.unwrap();

            dbg!(json);
        })
        .build();

    bot.start().await.unwrap();
}
