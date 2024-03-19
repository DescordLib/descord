use descord::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let mut client = Client::new(
        &std::env::var("DISCORD_TOKEN").unwrap(),
        GatewayIntent::MessageContent | GatewayIntent::GuildMessages,
        "!",
    )
    .await;

    client.register_commands([ping()]);

    client.login(Handler).await;
}

#[command(name = "pinger", prefix = "hah!")]
async fn ping(data: MessageData) {
    let clock = std::time::Instant::now();
    let msg = data.reply("Pong!").await;
    let latency = clock.elapsed().as_millis();

    msg.edit(format!("Pong! {}ms", latency)).await;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {}
