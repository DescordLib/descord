use nanoserde::{SerJson, DeJson};

use super::{author::Author, embed::Embed};

#[derive(DeJson, SerJson, Clone)]
pub struct MessageReference {
    pub tts: bool,
    pub timestamp: String,
    pub pinned: bool,
    pub mention_everyone: bool,
    pub flags: usize,
    pub embeds: Vec<Embed>,
    pub edited_timestamp: Option<String>,
    pub content: String,
    pub channel_id: String,
    pub author: Author,

    // TODO: mentions, mention_roles, attachments
}

