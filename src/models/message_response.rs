use json::object;
use nanoserde::{DeJson, SerJson};
use std::error::Error;

use super::allowed_mentions::AllowedMentions;
use super::attachment::Attachment;
use super::channel::Channel;
use super::components::Component;
use super::embed::Embed;
use super::guild::Member;
use crate::prelude::User;
use crate::utils;
use crate::{consts, Client};

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
    pub author: Option<User>,
    #[nserde(default)]
    pub referenced_message: Option<Box<Message>>,

    pub guild_id: Option<String>,

    pub id: String,

    pub member: Option<Member>,

    pub attachments: Vec<Attachment>,

    pub components: Vec<Component>,
    // TODO
    // mentions, mention_roles, member, etc.
}

impl Message {
    /// Reply to the message
    pub async fn reply(&self, data: impl Into<CreateMessageData>) -> Message {
        utils::reply(&self.id, &self.channel_id, data).await
    }

    /// Send a message in the same channel
    pub async fn send_in_channel(&self, data: impl Into<CreateMessageData>) -> Message {
        utils::send(&self.channel_id, data).await
    }

    /// Get the current channel
    pub async fn get_channel(&self) -> Result<Channel, Box<dyn std::error::Error>> {
        utils::fetch_channel(&self.channel_id).await
    }

    /// Get the message author
    pub async fn get_author(&self) -> Result<Member, Box<dyn Error>> {
        utils::fetch_member(
            self.guild_id.as_ref().unwrap(),
            &self.author.as_ref().unwrap().id,
        )
        .await
    }

    /// Delete this message
    pub async fn delete(&self) -> bool {
        utils::delete_message(&self.channel_id, &self.id).await
    }

    /// Delete this message after a certain amount of time
    pub async fn delete_after(&self, time: tokio::time::Duration) {
        tokio::time::sleep(time);
        self.delete().await;
    }

    pub async fn edit(&self, data: impl Into<CreateMessageData>) {
        utils::edit_message(&self.channel_id, &self.id, data).await;
    }

    /// Valid emoji formats: `name`, `name:id`
    pub async fn react(&self, emoji: &str) {
        utils::react(&self.channel_id, &self.id, emoji).await;
    }
}

#[derive(Default, Debug, DeJson, SerJson, Clone)]
pub struct CreateMessageData {
    pub content: String,
    pub tts: bool,
    pub embeds: Vec<Embed>,
    pub allowed_mentions: Option<AllowedMentions>,
    pub flags: Option<u32>,
    pub attachments: Vec<Attachment>,

    pub components: Vec<Component>,
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

        json::stringify(json)
    }

    pub fn add_components(mut self, components: Vec<Vec<Component>>) -> Self {
        let new: CreateMessageData = components.into();

        CreateMessageData {
            components: new.components,
            ..self
        }
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

impl From<&String> for CreateMessageData {
    fn from(value: &String) -> Self {
        Self {
            content: value.clone(),
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

impl From<Embed> for CreateMessageData {
    fn from(value: Embed) -> Self {
        CreateMessageData {
            embeds: vec![value],
            ..Default::default()
        }
    }
}

impl From<Vec<Attachment>> for CreateMessageData {
    fn from(value: Vec<Attachment>) -> Self {
        CreateMessageData {
            attachments: value,
            ..Default::default()
        }
    }
}

impl From<Attachment> for CreateMessageData {
    fn from(value: Attachment) -> Self {
        CreateMessageData {
            attachments: vec![value],
            ..Default::default()
        }
    }
}

impl From<AllowedMentions> for CreateMessageData {
    fn from(value: AllowedMentions) -> Self {
        CreateMessageData {
            allowed_mentions: Some(value),
            ..Default::default()
        }
    }
}

impl From<Vec<Vec<Component>>> for CreateMessageData {
    fn from(value: Vec<Vec<Component>>) -> Self {
        let components = value
            .iter()
            .map(|column| -> Component {
                let components = json::parse(&column.serialize_json()).unwrap();

                // TODO: improve this logic cause its kinda slow
                Component::deserialize_json(
                    &json::object! {
                        type: 1,
                        components: components,
                    }
                    .pretty(1),
                )
                .unwrap()
            })
            .collect::<Vec<_>>();

        CreateMessageData {
            components,
            ..Default::default()
        }
    }
}
