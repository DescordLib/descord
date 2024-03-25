/// A slash command
#[descord::slash]
pub async fn echo(
    interaction: Interaction,

    // This doc comment will be used as option description
    /// The message to be echo'ed
    message: String,
) {
    interaction.reply(message, false).await;
}
