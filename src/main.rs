use descord::prelude::*;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref BOT_ID: Mutex<String> = Mutex::new(String::new());
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap_or_else(|_| {
        eprintln!("Failed to load .env file");
        std::process::exit(1);
    });
    env_logger::init();

    let mut client = Client::new(
        &std::env::var("DISCORD_TOKEN").unwrap_or_else(|_| {
            eprintln!("DISCORD_TOKEN not found in .env file");
            std::process::exit(1);
        }),
        GatewayIntent::ALL,
        "!",
    )
    .await;

    register_all_commands!(client => []);
    register_all_events!(client => []);
    register_all_slash_commands!(client => []);

    client.login().await;
}

#[slash(name = "ping", description = "Replies with a cool pong!")]
async fn ping(interaction: Interaction) {
    interaction.reply("Pong!").await;
}

#[descord::event_handler]
async fn message_delete_raw(_: DeletedMessage) {
    println!("message deleted");
}

#[descord::event_handler]
async fn message_delete(data: Message) {
    println!("cached message deleted: {}", data.content);
}

// #[descord::event_handler]
// async fn guild_create(data: GuildCreate) {
//     println!("Guild: {}", data.name);
// }

#[descord::command]
async fn dm(msg: Message, message: Args) {
    msg.author.unwrap().send_dm(message.join(" ")).await;
}

#[descord::event_handler]
pub async fn message_create(message: Message) {
    if !message.content.starts_with("!!oogway") {
        return;
    }

    let (_, text) = message.content.split_once(" ").unwrap();
    let encoded_text = text.replace(" ", "%20");
    let embed = EmbedBuilder::new()
        .color(Color::Teal)
        .image(EmbedImage {
            url: format!("https://api.popcat.xyz/oogway?text={}", encoded_text),
            proxy_url: None,
            height: None,
            width: None,
        })
        .build();

    message
        .reply(CreateMessageData {
            embeds: vec![embed],
            ..Default::default()
        })
        .await;
}

#[command]
async fn echo(msg: Message, stuff: Args) {
    msg.reply(format!("Hello, {}", stuff.join(" "))).await;
}

// #[command(name = "ping")]
// async fn ping(msg: Message) {
//     let clock = std::time::Instant::now();
//     let msg = msg.reply("Pong!").await;
//     let latency = clock.elapsed().as_millis();
//
//     msg.edit(format!("Pong! :ping_pong:  `{}ms`", latency))
//         .await;
// }

#[command(name = "channel")]
async fn channel(msg: Message, channel: Channel) {
    msg.reply(format!("Channel: {}", channel.name)).await;
}

#[command(name = "user")]
async fn user(msg: Message, user: User) {
    msg.reply(format!(
        "name: {0}, id: {1} {2}",
        user.username,
        user.id,
        user.mention()
    ))
    .await;
}

#[command]
async fn av(msg: Message) {
    msg.reply(format!(
        "{}\n{}",
        msg.author
            .as_ref()
            .unwrap()
            .get_avatar_url(ImageFormat::Png, Some(16))
            .unwrap(),
        msg.author
            .as_ref()
            .unwrap()
            .get_avatar_url(ImageFormat::Png, Some(4096))
            .unwrap(),
    ))
    .await;
}

#[command]
async fn counter(msg: Message) {
    let msg = msg.send_in_channel("Count: 0").await;

    msg.react("⬆").await;
    msg.react("⬇").await;
}

#[command]
async fn react(msg: Message, emoji: String) {
    println!("reacting");
    msg.react(&emoji).await;
}

#[event_handler]
async fn ready(data: ReadyData) {
    println!(
        "Logged in as: {}#{}",
        data.user.username, data.user.discriminator
    );

    *BOT_ID.lock().await = data.user.id.into();
}

#[event_handler]
async fn reaction_add(reaction: Reaction) {
    if &reaction.user_id == BOT_ID.lock().await.as_str() {
        return;
    }

    let msg = reaction.get_message().await;
    let (counter_message, count) = msg.content.split_once(" ").unwrap();
    let mut count = count.parse::<isize>().unwrap();

    if reaction.emoji.name == "⬆" {
        count += 1;
        tokio::join!(
            reaction.remove_reaction(),
            msg.edit(format!("{counter_message} {count}"))
        );
    } else if reaction.emoji.name == "⬇" {
        count -= 1;
        tokio::join!(
            reaction.remove_reaction(),
            msg.edit(format!("{counter_message} {count}"))
        );
    }
}

#[command]
async fn button(message: Message) {
    let b1 : Component = ComponentBuilder::button(ButtonObject {
        style: ButtonStyle::Primary as _,
        label: Some("Click me".to_string()),
        custom_id: Some("btn1".to_string()),
        ..Default::default()
    })
    .unwrap();

    let b2: Component = ComponentBuilder::button(ButtonObject {
        style: ButtonStyle::Secondary as _,
        label: Some("Click me".to_string()),
        custom_id: Some("btn2".to_string()),
        ..Default::default()
    })
    .unwrap();

    let b3: Component = ComponentBuilder::button(ButtonObject {
        style: ButtonStyle::Danger as _,
        label: Some("Click me".to_string()),
        custom_id: Some("btn3".to_string()),
        disabled: true,
        ..Default::default()
    })
    .unwrap();

    message
        .reply(CreateMessageData {
            components: vec![
                vec![b1],
                vec![b2, b3],
            ],
            ..Default::default()
        })
        .await;
}
