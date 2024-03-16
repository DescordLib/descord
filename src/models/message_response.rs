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

    /// For replying/sending messages
    #[nserde(default, transparent)]
    pub(crate) token: Option<String>,
    // TODO
    // mentions, mention_roles, member, etc.
}

impl MessageData {
    pub fn reply(&self, data: CreateMessageData) {
        // let body = data.serialize_json();

        let body = json::stringify(object! {
            content: data.content,
            tts: data.tts,
            message_reference: {
                message_id: self.message_id.as_str(),
                guild_id: self.guild_id.as_str(),
            }
        });

        ureq::post(&format!(
            "{api}/channels/{channel_id}/messages",
            api = consts::API,
            channel_id = self.channel_id
        ))
        .set("Content-Type", "application/json")
        .set(
            "Authorization",
            &format!("Bot {}", self.token.as_ref().unwrap()),
        )
        .send_string(&body)
        .unwrap();
    }

    pub fn send_in_channel(&self, data: CreateMessageData) {
        let body = data.serialize_json();

        ureq::post(&format!(
            "{api}/channels/{channel_id}/messages",
            api = consts::API,
            channel_id = self.channel_id
        ))
        .set("Content-Type", "application/json")
        .set(
            "Authorization",
            &format!("Bot {}", self.token.as_ref().unwrap()),
        )
        .send_string(&body)
        .unwrap();
    }
}

#[derive(Default, Debug, Clone, SerJson)]
pub struct CreateMessageData<'a> {
    pub content: &'a str,
    pub tts: bool,
}

#[derive(DeJson, SerJson)]
pub struct ChannelId(String);
