#![allow(unused)]

use descord::prelude::*;

fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let mut client = Client::new(
        &std::env::var("DISCORD_TOKEN").unwrap(),
        GatewayIntent::MessageContent | GatewayIntent::GuildMessages,
    );

    client.login(Handler);
}

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ready_data: ReadyData) {
        println!(
            "Logged in as {}#{}",
            ready_data.user.username, ready_data.user.discriminator
        );
    }

    fn message_create(&self, message_data: MessageData) {
        if message_data.content == "ping" {
            message_data.reply(CreateMessageData {
                content: "Pong! (reply)",
                tts: false,
            });

            message_data.send_in_channel(CreateMessageData {
                content: "Pong!",
                tts: false,
            });
        }
    }
}
