use nanoserde::{DeJson, SerJson};

/// Represents a direct message channel.
#[derive(DeJson, SerJson, Debug)]
pub struct DirectMessageChannel {
    /// The unique ID of the direct message channel.
    pub id: String,
    /// The type of the channel.
    #[nserde(rename = "type")]
    pub type_: usize,
    /// The ID of the last message sent in the channel.
    pub last_message_id: Option<String>,
    /// The hash of the channel icon.
    #[nserde(rename = "icon")]
    pub icon_hash: Option<String>,
    /// The application ID of the channel.
    pub application_id: Option<String>,
    /// The flags of the channel.
    #[nserde(default)]
    pub flags: usize,
}
