use descord::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let mut client = Client::new(
        &std::env::var("DISCORD_TOKEN").unwrap(),
        GatewayIntent::MESSAGE_CONTENT | GatewayIntent::GUILD_MESSAGES | GatewayIntent::DIRECT_MESSAGES,
        "!",
    )
    .await;

    client.register_commands([ping(), dm(), say_hello()]);

    client.login(Handler).await;
}

#[command]
async fn dm(data: MessageData) {
    data.author.send_dm("You've asked for it!").await;
}

// automatic arg parsing
#[command]
async fn say_hello(data: MessageData, name: String) {
    data.reply(format!("Hello, {name}!")).await;
}

#[command(name = "ping")]
async fn ping(data: MessageData) {
    let clock = std::time::Instant::now();
    let msg = data.reply("Pong!").await;
    let latency = clock.elapsed().as_millis();

    msg.edit(format!("Pong! :ping_pong:  `{}ms`", latency))
        .await;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {}
