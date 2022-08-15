#[handle(MessageCreated)]
async fn handleMessageCreated(event: MessageCreated, sender: Sender) -> anyhow::Result<()> {
  api::create_message("おいすー").await?;
}

#[tokio_main]
async fn main() {
  handler::build()
    .mount(handleMessageCreated)
    .run()
    .await;
}

