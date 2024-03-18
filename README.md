# descord
Descord is a discord api wrapper.

## Example
```rust
use descord::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new(
        "DISCORD_TOKEN",
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
        }
    }
}
```
