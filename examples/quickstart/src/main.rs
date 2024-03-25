use descord::prelude::*;

mod commands;
mod events;

const DISCORD_TOKEN: &str = "TOKEN";

#[tokio::main]
async fn main() {
    let mut client = Client::new(
        DISCORD_TOKEN,
        GatewayIntent::NON_PRIVILEGED | GatewayIntent::MESSAGE_CONTENT, // message content is required for message commands
        "!", // leave it blank if you are not planning to use message commands
    )
    .await;

    // Commands and events should be registered manually
    client.register_commands(vec![commands::ping()]);
    client.register_slash_commands(vec![commands::echo()]).await;
    client.register_events(vec![events::ready()]);

    client.login().await;
}
