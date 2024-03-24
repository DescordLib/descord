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

    register_all!(client => []);
    client.login().await;
}

#[slash(name = "greet", description = "Get channel info")]
async fn ping(
    interaction: Interaction,
    /// The channel to get info about
    channel: Channel,
    /// The user to ping
    user: User,
) {
    interaction
        .reply(format!(
            "Channel: {}\nUser: {}",
            channel.name,
            user.mention()
        ))
        .await;

    interaction.followup("This is a followup message").await;
}

#[slash(name = "echo", description = "Echoes the input")]
async fn echo_slash(
    interaction: Interaction,
    /// The message to echo
    #[rename = "message"]
    msg: String,
) {
    interaction.defer().await;
    interaction.followup(msg).await;
}

#[event]
async fn message_delete_raw(_: DeletedMessage) {
    println!("message deleted");
}

#[event]
async fn message_delete(data: Message) {
    println!("cached message deleted: {}", data.content);
}

// #[event]
// async fn guild_create(data: GuildCreate) {
//     println!("Guild: {}", data.name);
// }

#[command]
async fn dm(msg: Message, message: Args) {
    msg.author.unwrap().send_dm(message.join(" ")).await;
}

#[command]
async fn echo(msg: Message, stuff: Args) {
    msg.reply(format!("Hello, {}", stuff.join(" "))).await;
}

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

#[event]
async fn ready(data: ReadyData) {
    println!(
        "Logged in as: {}#{}",
        data.user.username, data.user.discriminator
    );

    *BOT_ID.lock().await = data.user.id.into();
}

#[event]
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
async fn components(message: Message) {
    let b1: Component = ComponentBuilder::button(ButtonObject {
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

    let select = ComponentBuilder::select(SelectObject {
        select_type: SelectMenuType::StringSelect,
        placeholder: Some(String::from("String Select example")),
        custom_id: String::from("select"),
        options: Some(vec![
            SelectOption {
                label: "abc".to_string(),
                value: "abc".to_string(),
                ..Default::default()
            },
            SelectOption {
                label: "def".to_string(),
                value: "def".to_string(),
                ..Default::default()
            },
        ]),
        ..Default::default()
    })
    .unwrap();

    message
        .reply(CreateMessageData {
            components: vec![vec![b1], vec![b2, b3], vec![select]],
            ..Default::default()
        })
        .await;
}

// #[event]
// async fn interaction_create(interaction: Interaction) {
    // println!("interaction: {:?}", interaction);
    // int.message
    //     .unwrap()
    //     .send_in_channel(format!(
    //         "custom id: {}",
    //         int.data.unwrap().custom_id.unwrap()
    //     ))
    //     .await;
// }
