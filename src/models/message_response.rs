use json::object;
use nanoserde::{DeJson, SerJson};

use crate::utils;
use crate::{consts, Client};

use super::channel::Channel;
use super::components::Component;
use super::message_edit::MessageEditData;
use super::{author::Author, embed::Embed, message_reference::MessageReference};

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct MessageResponse {
    #[nserde(rename = "d")]
    pub data: Message,
}

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct Message {
    #[nserde(default)]
    pub tts: bool,

    #[nserde(default)]
    pub timestamp: Option<String>,

    #[nserde(default)]
    pub pinned: bool,

    #[nserde(default)]
    pub mention_everyone: bool,

    #[nserde(default)]
    pub flags: usize,

    pub edited_timestamp: Option<String>,

    #[nserde(default)]
    pub content: String,

    pub channel_id: String,
    #[nserde(default)]
    pub embeds: Vec<Embed>,
    pub author: Option<Author>,
    #[nserde(default)]
    pub referenced_message: Option<MessageReference>,

    pub guild_id: Option<String>,
    pub id: String,
    // TODO
    // mentions, mention_roles, member, etc.
}

impl Message {
    pub async fn reply(&self, data: impl Into<CreateMessageData>) -> Message {
        utils::reply(&self.id, &self.channel_id, data).await
    }

    pub async fn send_in_channel(&self, data: impl Into<CreateMessageData>) -> Message {
        utils::send(&self.channel_id, data).await
    }

    pub async fn get_channel(&self) -> Result<Channel, Box<dyn std::error::Error>> {
        utils::get_channel(&self.channel_id).await
    }

    pub async fn delete(&self) -> bool {
        utils::delete_message(&self.channel_id, &self.id).await
    }

    pub async fn delete_after(&self, time: u64) {
        tokio::time::sleep(tokio::time::Duration::from_millis(time)).await;
        self.delete().await;
    }

    pub async fn edit(&self, data: impl Into<MessageEditData>) {
        utils::edit_message(&self.channel_id, &self.id, data).await;
    }

    /// Valid emoji formats: `name`, `name:id`
    pub async fn react(&self, emoji: &str) {
        utils::react(&self.channel_id, &self.id, emoji).await;
    }
}

#[derive(Default, Debug, SerJson)]
pub struct CreateMessageData {
    pub content: String,
    pub tts: bool,
    pub embeds: Vec<Embed>,

    /// Column<Row<Component>>
    pub components: Vec<Vec<Component>>,
}

impl CreateMessageData {
    pub fn to_json(&self) -> String {
        let mut json = json::parse(&self.serialize_json()).unwrap();

        let components = self
            .components
            .iter()
            .map(|column| {
                let components = json::parse(&column.serialize_json()).unwrap();
                json::object! {
                    type: 1,
                    components: components,
                }
            })
            .collect::<Vec<_>>();

        json.remove("components");
        json.insert("components", components);

        json::stringify(json)
    }
}

impl From<String> for CreateMessageData {
    fn from(value: String) -> Self {
        Self {
            content: value,
            ..Default::default()
        }
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

impl From<Vec<Embed>> for CreateMessageData {
    fn from(value: Vec<Embed>) -> Self {
        assert!(
            value.len() <= 10,
            "A message can only contain up to 10 rich embeds"
        );

        CreateMessageData {
            embeds: value,
            ..Default::default()
        }
    }
}
