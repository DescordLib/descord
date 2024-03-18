use descord::prelude::*;
use std::time::Instant;

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

    async fn message_create(&self, msg: MessageData) {
        if msg.author.bot {
            return;
        }

        if msg.content == "!ping" {
            let clock = Instant::now();
            let new_msg = msg.reply("Pong").await;

            new_msg
                .edit(MessageEditData {
                    content: Some(format!(
                        "Pong! :ping_pong: `{}ms`",
                        clock.elapsed().as_millis()
                    )),
                    ..Default::default()
                })
                .await;
        }
    }
}
