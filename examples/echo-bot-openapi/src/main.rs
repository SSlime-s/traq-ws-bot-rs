use traq_ws_bot::{
    bot::builder,
    openapi::{self, models::PostMessageRequest},
    utils::create_configuration,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let bot_access_token = std::env::var("BOT_ACCESS_TOKEN").unwrap_or_else(|_| String::new());

    let bot = builder(&bot_access_token)
        .insert_resource(bot_access_token.clone())
        .on_direct_message_created_with_resource(|event, resource| async move {
            let post_message_request = PostMessageRequest {
                content: event.message.plain_text,
                embed: Some(false),
            };

            let configuration = create_configuration(resource.as_ref());

            let res = openapi::apis::message_api::post_message(
                &configuration,
                &event.message.channel_id,
                Some(post_message_request),
            )
            .await;

            match res {
                Ok(message) => {
                    dbg!("Success!: {:?}", message);
                }
                Err(e) => {
                    dbg!("Error: {}", e);
                }
            }
        })
        .build();

    bot.start().await.unwrap();
}
