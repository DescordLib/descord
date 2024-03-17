use descord::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let client = Client::new(
        &std::env::var("DISCORD_TOKEN").unwrap(),
        GatewayIntent::MessageContent | GatewayIntent::GuildMessages,
    ).await;

    client.login(Handler).await;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: &Context, ready_data: ReadyData) {
        println!(
            "Logged in as: {}#{}",
            ready_data.user.username, ready_data.user.discriminator
        );
    }

    async fn message_create(&self, ctx: &Context, message_data: MessageData) {
        if message_data.content == ".ping" {
            ctx.reply(&message_data, "Pong (reply)").await;
            ctx.send(&message_data.channel_id, "Pong").await;
        }
    }
}
