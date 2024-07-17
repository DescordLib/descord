use descord::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::init();

    let mut client = Client::new(
        &std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment"),
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
    /// The user to mention, optional
    user: Option<User>,
) {
    if let Some(user) = user {
        interaction
            .reply(
                format!("Hello, {}! You are in {}", user.mention, channel.mention),
                false,
            )
            .await;
    } else {
        interaction
            .reply(format!("You are in {}", channel.mention), false)
            .await;
    }
}

async fn auto_cmp(value: String) -> Vec<String> {
    let options = vec!["fireplank", "wizard"];
    options
        .into_iter()
        .filter(|o| o.starts_with(&value))
        .map(|o| o.to_string())
        .collect()
}

#[slash(name = "echo", description = "Echoes the input")]
#[permissions = "administrator"]
async fn echo_slash(interaction: Interaction, #[autocomplete = auto_cmp] message: String) {
    interaction.defer().await;
    interaction.followup(message).await;
}

#[slash(name = "whisper", description = "Respond with ephemeral message")]
async fn whisper(interaction: Interaction) {
    interaction
        .reply("This is an ephemeral message", true)
        .await;
}

#[event]
async fn message_delete_raw(_: DeletedMessage) {
    println!("message deleted");
}

#[command(permissions = "administrator")]
async fn echo(msg: Message, stuff: String) {
    msg.reply(format!("Hello, {}", stuff)).await;
}

#[slash(description = "Get a user's avatar")]
async fn avatar(interaction: Interaction, #[doc = "User to fetch avatar from"] user: Option<User>) {
    let member = interaction.member.as_ref().unwrap();
    let (username, avatar) = match user {
        Some(user) => (
            &user.username,
            user.get_avatar_url(ImageFormat::WebP, None).unwrap(),
        ),
        None => (
            member
                .nick
                .as_ref()
                .unwrap_or_else(|| &member.user.as_ref().unwrap().username),
            member.get_avatar_url(ImageFormat::WebP, None).unwrap(),
        ),
    };

    let embed = EmbedBuilder::new()
        .color(Color::Blue)
        .title(&format!("{}'s avatar", username))
        .image(avatar, None, None)
        .build();

    interaction.reply(embed, false).await;
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
}

#[event]
async fn reaction_add(reaction: Reaction) {
    if reaction.member.clone().unwrap().user.unwrap().bot {
        return;
    }

    let msg = reaction.get_message().await.unwrap();
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
