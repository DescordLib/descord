# descord
Descord is a discord api wrapper.

## Example
```rust
use descord::prelude::*;

#[tokio::main]
async fn main() {
    let mut client = Client::new(
        "TOKEN",
        GatewayIntent::MessageContent | GatewayIntent::GuildMessages,
    )
    .await;

    client.register_commands([ping()]);
    client.login(Handler).await;
}

#[descord::command("!ping")]
async fn ping(data: MessageData) {
    let clock = std::time::Instant::now();
    let msg = data.reply("Pong!").await;
    let latency = clock.elapsed().as_millis();

    msg.edit(format!("Pong! {}ms", latency)).await;
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
}
```
