use descord::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let mut client = Client::new(
        &std::env::var("DISCORD_TOKEN").unwrap(),
        GatewayIntent::MESSAGE_CONTENT
            | GatewayIntent::GUILD_MESSAGES
            | GatewayIntent::DIRECT_MESSAGES,
        "!",
    )
    .await;

    register_all_commands!();
    client.login(Handler).await;
}

#[command]
async fn dm(data: MessageData) {
    data.author.send_dm("You've asked for it!").await;
}

#[command]
async fn say_hello(data: MessageData, name: std::String) {
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

#[command(name = "echo")]
async fn echo(data: MessageData) {
    let msg = data.reply("Echo!").await;
    msg.delete_after(5000).await;
}

#[command(name = "channel")]
async fn channel(data: MessageData, channel: Channel) {
    data.reply(format!("Channel: {}", channel.name)).await;
}

#[command(name = "user")]
async fn user(data: MessageData, user: User) {
    data.reply(format!(
        "name: {0}, id: {1} {2}",
        user.username,
        user.id,
        user.mention()
    ))
    .await;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {}
