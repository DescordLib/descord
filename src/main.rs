use descord::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let client = Client::new(
        &std::env::var("DISCORD_TOKEN").unwrap(),
        GatewayIntent::MessageContent | GatewayIntent::GuildMessages,
    )
    .await;

    client.login(Handler).await;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ready_data: ReadyData) {
        println!(
            "Logged in as: {}#{}",
            ready_data.user.username, ready_data.user.discriminator
        );
    }

    async fn message_create(&self, data: MessageData) {
        if data.author.bot {
            return;
        }
    }
}

#[command("ping")]
async fn ping(data: descord::prelude::MessageData) {
    let clock = std::time::Instant::now();
    let _ = data.get_channel().await.unwrap();
    let latency = clock.elapsed().as_millis();

    data.reply(format!("Pong! {}ms", latency)).await;
}
