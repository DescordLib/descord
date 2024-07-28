# descord
Descord is a minimal, easy to use discord api wrapper.

## Example
```rust
use descord::prelude::*;

#[tokio::main]
async fn main() {
    let mut client = Client::new(
        "TOKEN",
        GatewayIntent::NON_PRIVILEGED
            // for message commands
            | GatewayIntent::MESSAGE_CONTENT,
        "!", // default prefix for message command
    )
    .await;
    
    // register commands, events and slash commands manually
    client.register_commands(vec![echo()]);
    client.register_events(vec![ready()]);
    client.register_slash_commands(vec![avatar()]).await;


    // alternatively you can do this, this is neat but
    // it is very counterintuitive since it read
    // the files and find functions marked with
    // proc macros

    // register_all!(client => ["src/main.rs", "src/file1.rs"]);


    // start the bot!
    client.login().await;
}

// An event handler
#[event]
async fn ready(data: ReadyData) {
    println!(
        "Logged in as {}#{}",
        data.user.username, data.user.discriminator
    )
}

// A message command
//
// you can also do `#[command(prefix = "new_prefix")]` to change
// the command prefix for this command to change
// the command prefix for this command
#[command]
async fn echo(
    /// information about the messgae
    msg: Message,
    /// some types can be parsed automatically
    echo_what: String,
) {
    msg.reply(echo_what).await;
}

// A slash command
#[slash(description = "Get a user's avatar")]
async fn avatar(
    interaction: Interaction,
    #[doc = "User to fetch avatar from"] user: Option<User>,
) {
    let member = interaction.member.as_ref().unwrap();
    let (username, avatar_url) = match user {
        Some(user) => (
            &user.username,
            user.get_avatar_url(ImageFormat::WebP, None).unwrap(),
        ),

        _ => (
            &member.user.as_ref().unwrap().username,
            member.get_avatar_url(ImageFormat::WebP, None).unwrap(),
        ),
    };

    // Creating an embed
    let embed = EmbedBuilder::new()
        .color(Color::Orange)
        .title(&format!("{username}'s avatar"))
        .image(avatar_url, None, None)
        .build();

    interaction.reply(embed, false).await;
}
```
