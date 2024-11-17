use json::object;
use nanoserde::{DeJson, SerJson};
use std::error::Error;

use super::allowed_mentions::AllowedMentions;
use super::attachment::{Attachment, AttachmentPayload};
use super::channel::Channel;
use super::components::Component;
use super::embed::Embed;
use super::guild::{Guild, Member};
use crate::prelude::User;
use crate::utils;
use crate::{consts, Client};

/// Represents a response to a message.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct MessageResponse {
    /// The message data.
    #[nserde(rename = "d")]
    pub data: Message,
}

/// Represents a message.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct Message {
    /// Whether the message is a TTS message.
    #[nserde(default)]
    pub tts: bool,
    /// The timestamp of the message.
    #[nserde(default)]
    pub timestamp: Option<String>,
    /// Whether the message is pinned.
    #[nserde(default)]
    pub pinned: bool,
    /// Whether the message mentions everyone.
    #[nserde(default)]
    pub mention_everyone: bool,
    /// The flags of the message.
    #[nserde(default)]
    pub flags: usize,
    /// The timestamp of when the message was edited.
    pub edited_timestamp: Option<String>,
    /// The content of the message.
    #[nserde(default)]
    pub content: String,
    /// The ID of the channel where the message was sent.
    pub channel_id: String,
    /// The embeds of the message.
    #[nserde(default)]
    pub embeds: Vec<Embed>,
    /// The author of the message.
    pub author: Option<User>,
    /// The message that is being replied to.
    #[nserde(default)]
    pub referenced_message: Option<Box<Message>>,
    /// The ID of the guild where the message was sent.
    pub guild_id: Option<String>,
    /// The ID of the message.
    pub id: String,
    /// The member who sent the message.
    pub member: Option<Member>,
    /// The attachments of the message.
    pub attachments: Vec<Attachment>,
    /// The components of the message.
    pub components: Vec<Component>,
    // TODO
    // mentions, mention_roles, member, etc.
}

impl Message {
    /// Reply to the message.
    ///
    /// # Arguments
    ///
    /// * `data` - The data for the reply.
    ///
    /// # Examples
    ///
    /// ```
    /// message.reply("Hello, world!").await;
    /// ```
    pub async fn reply(&self, data: impl Into<CreateMessageData>) -> Message {
        utils::send(&self.channel_id, Some(&self.id), data).await
    }

    /// Send a message in the same channel.
    ///
    /// # Arguments
    ///
    /// * `data` - The data for the message.
    ///
    /// # Examples
    ///
    /// ```
    /// message.send_in_channel("Hello, world!").await;
    /// ```
    pub async fn send_in_channel(&self, data: impl Into<CreateMessageData>) -> Message {
        utils::send(&self.channel_id, None, data).await
    }

    /// Get the current channel.
    ///
    /// # Examples
    ///
    /// ```
    /// let channel = message.get_channel().await?;
    /// ```
    pub async fn get_channel(&self) -> Result<Channel, Box<dyn std::error::Error>> {
        utils::fetch_channel(&self.channel_id).await
    }

    /// Sends typing indicator.
    pub async fn send_typing(&self) -> Result<(), Box<dyn Error>> {
        utils::send_typing(&self.channel_id).await
    }

    /// Get the message author.
    ///
    /// # Examples
    ///
    /// ```
    /// let author = message.get_author().await?;
    /// ```
    pub async fn get_author(&self) -> Result<Member, Box<dyn Error>> {
        utils::fetch_member(
            self.guild_id.as_ref().unwrap(),
            &self.author.as_ref().unwrap().id,
        )
        .await
    }

    /// Get the guild in which the message was sent.
    ///
    /// # Examples
    ///
    /// ```
    /// let guild = message.get_guild().await?;
    /// ```
    pub async fn get_guild(&self) -> Result<Guild, Box<dyn Error>> {
        utils::fetch_guild(self.guild_id.as_ref().unwrap()).await
    }

    /// Get the message that is being replied to.
    ///
    /// # Examples
    ///
    /// ```
    /// let referenced_message = message.get_message_reference().await;
    /// ```
    pub async fn get_message_reference(&self) -> Option<Message> {
        if let Some(ref message) = self.referenced_message {
            Some(*message.clone())
        } else {
            None
        }
    }

    /// Delete this message.
    ///
    /// # Examples
    ///
    /// ```
    /// message.delete().await;
    /// ```
    pub async fn delete(&self) -> bool {
        utils::delete_message(&self.channel_id, &self.id).await
    }

    /// Delete this message after a certain amount of time.
    ///
    /// # Arguments
    ///
    /// * `time` - The duration to wait before deleting the message.
    ///
    /// # Examples
    ///
    /// ```
    /// message.delete_after(tokio::time::Duration::from_secs(10)).await;
    /// ```
    pub async fn delete_after(&self, time: tokio::time::Duration) {
        tokio::time::sleep(time);
        self.delete().await;
    }

    /// Edit the message.
    ///
    /// # Arguments
    ///
    /// * `data` - The new data for the message.
    ///
    /// # Examples
    ///
    /// ```
    /// message.edit("Edited message").await;
    /// ```
    pub async fn edit(&self, data: impl Into<CreateMessageData>) {
        utils::edit_message(&self.channel_id, &self.id, data).await;
    }

    /// React to the message with an emoji.
    ///
    /// # Arguments
    ///
    /// * `emoji` - The emoji to react with.
    ///
    /// # Examples
    ///
    /// ```
    /// message.react("👍").await;
    /// ```
    pub async fn react(&self, emoji: &str) {
        utils::react(&self.channel_id, &self.id, emoji).await;
    }
}

/// Data for creating a message.
#[derive(Default, Debug, DeJson, SerJson, Clone)]
pub struct CreateMessageData {
    /// The content of the message.
    pub content: String,
    /// Whether the message is a TTS message.
    pub tts: bool,
    /// The embeds of the message.
    pub embeds: Vec<Embed>,
    /// The allowed mentions in the message.
    pub allowed_mentions: Option<AllowedMentions>,
    /// The flags of the message.
    pub flags: Option<u32>,
    /// The components of the message.
    pub components: Vec<Component>,
    /// The attachments of the message.
    #[nserde(transparent)]
    pub attachments: Vec<AttachmentPayload>,
}

impl CreateMessageData {
    /// Converts the message data to JSON.
    ///
    /// # Examples
    ///
    /// ```
    /// let json = message_data.to_json();
    /// ```
    pub fn to_json(&self) -> String {
        let mut json = json::parse(&self.serialize_json()).unwrap();

        json.remove("attachments");

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

    /// Adds components to the message data.
    ///
    /// # Arguments
    ///
    /// * `components` - The components to add.
    ///
    /// # Examples
    ///
    /// ```
    /// message_data.add_components(vec![vec![component1, component2]]);
    /// ```
    pub fn add_components(mut self, components: Vec<Vec<Component>>) -> Self {
        let new: CreateMessageData = components.into();

        CreateMessageData {
            components: new.components,
            ..self
        }
    }
}

impl From<String> for CreateMessageData {
    /// Converts a string to `CreateMessageData`.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value.
    ///
    /// # Examples
    ///
    /// ```
    /// let message_data: CreateMessageData = "Hello, world!".into();
    /// ```
    fn from(value: String) -> Self {
        Self {
            content: value,
            ..Default::default()
        }
    }
}

impl From<&String> for CreateMessageData {
    /// Converts a string reference to `CreateMessageData`.
    ///
    /// # Arguments
    ///
    /// * `value` - The string reference.
    ///
    /// # Examples
    ///
    /// ```
    /// let message_data: CreateMessageData = (&"Hello, world!".to_string()).into();
    /// ```
    fn from(value: &String) -> Self {
        Self {
            content: value.clone(),
            ..Default::default()
        }
    }
}

impl From<&str> for CreateMessageData {
    /// Converts a string slice to `CreateMessageData`.
    ///
    /// # Arguments
    ///
    /// * `value` - The string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let message_data: CreateMessageData = "Hello, world!".into();
    /// ```
    fn from(value: &str) -> Self {
        Self {
            content: value.to_owned(),
            ..Default::default()
        }
    }
}

impl From<Vec<Embed>> for CreateMessageData {
    /// Converts a vector of embeds to `CreateMessageData`.
    ///
    /// # Arguments
    ///
    /// * `value` - The vector of embeds.
    ///
    /// # Examples
    ///
    /// ```
    /// let message_data: CreateMessageData = vec![embed1, embed2].into();
    /// ```
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
    /// Converts an embed to `CreateMessageData`.
    ///
    /// # Arguments
    ///
    /// * `value` - The embed.
    ///
    /// # Examples
    ///
    /// ```
    /// let message_data: CreateMessageData = embed.into();
    /// ```
    fn from(value: Embed) -> Self {
        CreateMessageData {
            embeds: vec![value],
            ..Default::default()
        }
    }
}

impl From<AllowedMentions> for CreateMessageData {
    /// Converts allowed mentions to `CreateMessageData`.
    ///
    /// # Arguments
    ///
    /// * `value` - The allowed mentions.
    ///
    /// # Examples
    ///
    /// ```
    /// let message_data: CreateMessageData = allowed_mentions.into();
    /// ```
    fn from(value: AllowedMentions) -> Self {
        CreateMessageData {
            allowed_mentions: Some(value),
            ..Default::default()
        }
    }
}

impl From<Vec<Vec<Component>>> for CreateMessageData {
    /// Converts a vector of component vectors to `CreateMessageData`.
    ///
    /// # Arguments
    ///
    /// * `value` - The vector of component vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// let message_data: CreateMessageData = vec![vec![component1, component2]].into();
    /// ```
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

impl From<AttachmentPayload> for CreateMessageData {
    /// Converts an attachment payload to `CreateMessageData`.
    ///
    /// # Arguments
    ///
    /// * `value` - The attachment payload.
    ///
    /// # Examples
    ///
    /// ```
    /// let message_data: CreateMessageData = attachment_payload.into();
    /// ```
    fn from(value: AttachmentPayload) -> Self {
        CreateMessageData {
            attachments: vec![value],
            ..Default::default()
        }
    }
}

impl From<Vec<AttachmentPayload>> for CreateMessageData {
    /// Converts a vector of attachment payloads to `CreateMessageData`.
    ///
    /// # Arguments
    ///
    /// * `value` - The vector of attachment payloads.
    ///
    /// # Examples
    ///
    /// ```
    /// let message_data: CreateMessageData = vec![attachment_payload1, attachment_payload2].into();
    /// ```
    fn from(value: Vec<AttachmentPayload>) -> Self {
        CreateMessageData {
            attachments: value,
            ..Default::default()
        }
    }
}
