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
        &std::env::var("DISCORD_TOKEN").unwrap(),
        GatewayIntent::ALL,
        "!",
    )
    .await;

    register_all_commands!(client => []);

    client.register_events(vec![ready()]);
    client.login().await;
}

#[command]
async fn dm(msg: Message) {
    msg.author.send_dm("You've asked for it!").await;
}

#[event_handler(ready)]
async fn ready(r: ReadyData) {
    println!("Logged in as: {:?}", r);
}
