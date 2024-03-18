use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson)]
pub struct Channel {
    pub last_message_id: String,
    pub flags: usize,
    pub guild_id: String,
    pub parent_id: String,
    pub topic: Option<String>,
    pub rate_limit_per_user: usize,
    pub position: u32,
    pub nsfw: bool,
    pub name: String,
    pub id: String,

    #[nserde(rename = "type")]
    pub channel_type: usize,
}
