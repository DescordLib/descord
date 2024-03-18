const IMAGE: &str = "https://images-ext-1.discordapp.net/external/O6IOVg-aIK9hoSwd1UtEQmgrAmxlpyzYjS2rW6shLgk/%3Fsize%3D4096/https/cdn.discordapp.com/avatars/815189874478546954/6a7a25edccdcabbe9f29558a0b6a7f00.png?format=webp&quality=lossless";

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
    async fn ready(&self, _ctx: &Context, ready_data: ReadyData) {
        println!(
            "Logged in as: {}#{}",
            ready_data.user.username, ready_data.user.discriminator
        );
    }

    async fn message_create(&self, ctx: &Context, data: MessageData) {
        if data.author.bot {
            return;
        }

        if data.content == "test" && data.author.user_id == "815189874478546954" {
            let embed = EmbedBuilder::new()
                .title("An Embed")
                .description("Embed description")
                .color(Color::Teal)
                .field("field 1", "value 1", false)
                .field("field 2", "value 2", false)
                .field("field 3", "value 3", false)
                .footer("Footer text", Some(IMAGE.to_owned()), None)
                .build();

            ctx.reply(
                &data,
                CreateMessageData {
                    embeds: vec![embed],
                    ..Default::default()
                },
            )
            .await;
        }
    }
}
