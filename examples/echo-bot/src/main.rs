use traq_ws_bot::bot::builder;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let bot_access_token = std::env::var("BOT_ACCESS_TOKEN").unwrap_or_else(|_| String::new());

    let bot = builder(bot_access_token)
        .on_direct_message_created(|event| {
            println!("{:?}", event);
        })
        .build();

    bot.start().await.unwrap();
}
