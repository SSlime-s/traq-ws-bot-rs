use reqwest::header::CONTENT_TYPE;
use traq_ws_bot::{
    bot::{builder, TRAQ_ORIGIN},
    utils::create_client,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let bot_access_token = std::env::var("BOT_ACCESS_TOKEN").unwrap_or_else(|_| String::new());

    let bot = builder(&bot_access_token)
        .insert_resource(bot_access_token.clone())
        .on_direct_message_created_with_resource(|event, resource| async move {
            let client = create_client(resource.as_ref());
            let url = format!(
                "{}/api/v3/channels/{}/messages",
                TRAQ_ORIGIN, event.message.channel_id
            );

            let body = format!(
                "{{\"content\": \"{}\", \"embed\": false}}",
                event.message.plain_text
            );

            let res = client
                .post(&url)
                .header(CONTENT_TYPE, "application/json")
                .body(body)
                .send()
                .await
                .unwrap();

            dbg!(res.status());
        })
        .build();

    bot.start().await.unwrap();
}
