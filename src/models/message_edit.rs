use nanoserde::{DeJson, SerJson};

use super::embed::Embed;

#[derive(DeJson, SerJson, Default, Debug)]
pub struct MessageEditData {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub flags: Option<usize>,
    // TODO: allowed_mentions, attachments, files, components
}

impl From<String> for MessageEditData {
    fn from(value: String) -> Self {
        Self {
            content: Some(value),
            ..Default::default()
        }
    }
}

impl From<&str> for MessageEditData {
    fn from(value: &str) -> Self {
        Self {
            content: Some(value.to_owned()),
            ..Default::default()
        }
    }
}
