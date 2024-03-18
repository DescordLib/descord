use nanoserde::{DeJson, SerJson};
use crate::utils;

use super::message_response::CreateMessageData;

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

impl Channel {
    pub async fn send_message(&self, data: impl Into<CreateMessageData>) {
        utils::send(&self.id, data);
    }
}
