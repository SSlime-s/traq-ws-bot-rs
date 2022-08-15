use traq_ws_bot_rs::bot::TraqBot;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let bot_access_token = std::env::var("BOT_ACCESS_TOKEN").unwrap_or_else(|_| String::new());

    let bot = TraqBot::new(bot_access_token, "ws://localhost:3000").on_message_created(|event| {
        println!("{:?}", event);
    });
    bot.start().await.unwrap();
}
