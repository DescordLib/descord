use json::object;
use nanoserde::{DeJson, SerJson};

use crate::{consts, Client};

use super::{author::Author, embed_data::EmbedData, message_reference::MessageReference};

#[derive(DeJson, SerJson)]
pub struct MessageResponse {
    #[nserde(rename = "d")]
    pub data: MessageData,
}

#[derive(DeJson, SerJson)]
pub struct MessageData {
    pub tts: bool,
    pub timestamp: String,
    pub pinned: bool,
    pub mention_everyone: bool,

    pub flags: usize,
    pub edited_timestamp: Option<String>,
    pub content: String,
    pub channel_id: String,
    pub embeds: Vec<EmbedData>,
    pub author: Author,

    #[nserde(default)]
    pub referenced_message: Option<MessageReference>,

    pub guild_id: String,

    #[nserde(rename = "id")]
    pub message_id: String,
    // TODO
    // mentions, mention_roles, member, etc.
}

#[derive(Default, Debug, Clone, SerJson)]
pub struct CreateMessageData {
    pub content: String,
    pub tts: bool,
}

impl From<String> for CreateMessageData {
    fn from(value: String) -> Self {
        Self {
            content: value,
            ..Default::default()
        }
    }
}

impl <'a> From<&'a CreateMessageData> for CreateMessageData {
    fn from(value: &'a CreateMessageData) -> Self {
        value.clone()
    }
}

impl From<&str> for CreateMessageData {
    fn from(value: &str) -> Self {
        Self {
            content: value.to_owned(),
            ..Default::default()
        }
    }
}
