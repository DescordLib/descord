# descord
Descord is a discord api wrapper.

## Example
```rust
use std::time::Instant;
use descord::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new(
        "TOKEN",
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
```
