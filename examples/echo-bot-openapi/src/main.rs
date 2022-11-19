use traq_ws_bot::{
    bot::builder,
    openapi::{self, models::PostMessageRequest},
    utils::{create_configuration, RateLimiter},
};

struct Resource {
    bot_access_token: String,
    limiter: RateLimiter,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let bot_access_token = std::env::var("BOT_ACCESS_TOKEN").unwrap_or_else(|_| String::new());
    let limiter = RateLimiter::default();
    let resource = Resource {
        bot_access_token: bot_access_token.clone(),
        limiter,
    };

    let bot = builder(&bot_access_token)
        .insert_resource(resource)
        .on_direct_message_created_with_resource(|event, resource| async move {
            let post_message_request = PostMessageRequest {
                content: event.message.plain_text,
                embed: Some(false),
            };

            let configuration = create_configuration(&resource.bot_access_token);

            if !resource.limiter.try_acquire().await {
                dbg!("Rate limit exceeded");
                return;
            }

            let res = openapi::apis::message_api::post_message(
                &configuration,
                &event.message.channel_id,
                Some(post_message_request),
            )
            .await;

            match res {
                Ok(message) => {
                    dbg!("Success!", message);
                }
                Err(e) => {
                    dbg!("Error", e);
                }
            }
        })
        .build();

    bot.start().await.unwrap();
}
