use nanoserde::{SerJson, DeJson};

use super::{embed_data::EmbedData, author::Author};

#[derive(DeJson, SerJson)]
pub struct MessageReference {
    pub tts: bool,
    pub timestamp: String,
    pub pinned: bool,
    pub mention_everyone: bool,
    pub flags: usize,
    pub embeds: Vec<EmbedData>,
    pub edited_timestamp: Option<String>,
    pub content: String,
    pub channel_id: String,
    pub author: Author,

    // TODO: mentions, mention_roles, attachments
}

