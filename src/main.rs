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

    client.login().await;
}

#[event_handler(ready)]
async fn ready(data: ReadyData) {
    println!(
        "Logged in as: {}#{}",
        data.user.username, data.user.discriminator
    );

    *BOT_ID.lock().await = data.user.id.into();
}

#[event_handler(reaction_add)]
async fn reaction_add(data: ReactionData) {
    if &data.user_id == BOT_ID.lock().await.as_str() {
        return;
    }

    let msg = data.get_message().await;
    let (counter_message, count) = msg.content.split_once(" ").unwrap();
    let mut count = count.parse::<isize>().unwrap();

    if data.emoji.name == "⬆" {
        count += 1;
        tokio::join!(
            data.remove_reaction(),
            msg.edit(format!("{counter_message} {count}"))
        );
    } else if data.emoji.name == "⬇" {
        count -= 1;
        tokio::join!(
            data.remove_reaction(),
            msg.edit(format!("{counter_message} {count}"))
        );
    }
}

#[event_handler(guild_create)]
async fn guild_create(data: GuildCreateData) {
    println!("Guild: {}", data.name);
}

#[command]
async fn dm(msg: Message) {
    msg.author.send_dm("You've asked for it!").await;
}

#[command]
async fn say_hello(msg: Message, name: std::String) {
    msg.reply(format!("Hello, {name}!")).await;
}

#[command(name = "ping")]
async fn ping(msg: Message) {
    let clock = std::time::Instant::now();
    let msg = msg.reply("Pong!").await;
    let latency = clock.elapsed().as_millis();

    msg.edit(format!("Pong! :ping_pong:  `{}ms`", latency))
        .await;
}

#[command(name = "echo")]
async fn echo(msg: Message) {
    let msg = msg.reply("Echo!").await;
    msg.delete_after(5000).await;
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
