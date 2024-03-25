/// A message command
/// Will be triggered when someone enters `!ping` in the chat
#[descord::command]
pub async fn ping(message: Message) {
    let start = std::time::Instant::now();
    let reply = message.reply("Pong!").await;

    reply
        .edit(format!(
            "Pong! :ping_pong: `{}ms`",
            start.elapsed().as_millis()
        ))
        .await;
}

