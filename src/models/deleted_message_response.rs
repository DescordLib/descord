use nanoserde::{DeJson, SerJson};

/// Represents a response for a deleted message.
#[derive(DeJson, SerJson, Debug)]
pub struct DeletedMessageResponse {
    /// The deleted message data.
    #[nserde(rename = "d")]
    pub data: DeletedMessage,
}

/// Represents a deleted message.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct DeletedMessage {
    /// The unique ID of the deleted message.
    #[nserde(rename = "id")]
    pub message_id: String,
    /// The ID of the channel where the message was deleted.
    pub channel_id: String,
    /// The ID of the guild where the message was deleted.
    pub guild_id: String,
}
