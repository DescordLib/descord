use json::JsonValue;
use std::collections::HashMap;

use crate::consts::*;
use crate::models::allowed_mentions::AllowedMentions;
use crate::models::guild::Member;
use crate::prelude::{Component, Embed};
use crate::utils::request;
use nanoserde::{DeJson, SerJson};
use reqwest::Method;

use super::guild::{Guild, PartialGuild};
use super::message_response::CreateMessageData;
use super::{channel::Channel, message_response::Message, user::User};

/// Payload for an interaction response.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct InteractionResponsePayload {
    /// The interaction data.
    #[nserde(rename = "d")]
    pub data: Interaction,
}

/// Represents an interaction with the Discord API.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct Interaction {
    /// The interaction ID.
    pub id: String,
    /// The application ID.
    pub application_id: String,
    /// The interaction type.
    #[nserde(rename = "type")]
    pub type_: u32,
    /// The interaction data.
    pub data: Option<InteractionData>,
    /// The channel where the interaction was sent.
    pub channel: Option<Channel>,
    /// The channel ID.
    pub channel_id: Option<String>,
    /// The member who invoked the interaction.
    pub member: Option<Member>,
    /// The interaction token.
    pub token: String,
    /// The message associated with the interaction.
    pub message: Option<Message>,
    /// The application permissions.
    pub app_permissions: String,
    /// The locale of the interaction.
    pub locale: Option<String>,
    /// The guild locale of the interaction.
    pub guild_locale: Option<String>,
    /// The context of the interaction.
    pub context: Option<u32>,
    /// The guild ID where the interaction was sent.
    pub guild_id: String,
    /// The user who invoked the interaction, if in a DM.
    pub user: Option<User>,
}

impl Interaction {
    /// Sends a reply to the interaction.
    ///
    /// # Arguments
    ///
    /// * `response` - The response data.
    /// * `ephemeral` - Whether the response should be ephemeral.
    ///
    /// # Examples
    ///
    /// ```
    /// interaction.reply("Hello, world!", true).await;
    /// ```
    pub async fn reply(&self, response: impl Into<CreateMessageData>, ephemeral: bool) {
        let mut message_data: CreateMessageData = response.into();
        ephemeral.then(|| message_data.flags = Some(64));

        let response = InteractionResponse {
            type_: 4,
            data: Some(message_data),
        };
        let json_response = SerJson::serialize_json(&response);

        request(
            Method::POST,
            format!("interactions/{}/{}/callback", self.id, self.token).as_str(),
            Some(JsonValue::from(json_response)),
        )
        .await;
    }

    /// Defers the interaction response.
    ///
    /// # Examples
    ///
    /// ```
    /// interaction.defer().await;
    /// ```
    pub async fn defer(&self) {
        let response = InteractionResponse {
            type_: 5,
            data: None,
        };
        let json_response = SerJson::serialize_json(&response);

        request(
            Method::POST,
            format!("interactions/{}/{}/callback", self.id, self.token).as_str(),
            Some(JsonValue::from(json_response)),
        )
        .await;
    }

    /// Sends a follow-up message to the interaction.
    ///
    /// # Arguments
    ///
    /// * `response` - The follow-up message content.
    ///
    /// # Examples
    ///
    /// ```
    /// interaction.followup("Follow-up message").await;
    /// ```
    pub async fn followup<S: AsRef<str>>(&self, response: S) {
        request(
            Method::POST,
            format!("webhooks/{}/{}", self.application_id, self.token).as_str(),
            Some(json::object! {
                content: response.as_ref(),
            }),
        )
        .await;
    }

    /// Edits the original interaction response.
    ///
    /// # Arguments
    ///
    /// * `response` - The new response data.
    ///
    /// # Examples
    ///
    /// ```
    /// interaction.edit_original("Edited message").await;
    /// ```
    pub async fn edit_original(&self, response: impl Into<CreateMessageData>) {
        let response: CreateMessageData = response.into();

        let resp = request(
            Method::PATCH,
            format!(
                "webhooks/{}/{}/messages/@original",
                self.application_id, self.token
            )
            .as_str(),
            Some(json::parse(&response.serialize_json()).unwrap()),
        )
        .await;

        println!("{}", resp.text().await.unwrap());
    }

    /// Deletes the original interaction response.
    ///
    /// # Examples
    ///
    /// ```
    /// interaction.delete_original().await;
    /// ```
    pub async fn delete_original(&self) {
        request(
            Method::DELETE,
            format!(
                "webhooks/{}/{}/messages/@original",
                self.application_id, self.token
            )
            .as_str(),
            None,
        )
        .await;
    }
}

/// Data associated with an interaction.
#[derive(DeJson, SerJson, Clone, Debug, Default)]
pub struct InteractionData {
    /// The custom ID of the interaction.
    pub custom_id: Option<String>,
    /// The component type of the interaction.
    pub component_type: Option<u32>,
    /// The ID of the interaction.
    pub id: Option<String>,
    /// The command name of the interaction.
    #[nserde(rename = "name")]
    pub command_name: Option<String>,
    /// The type of the interaction.
    #[nserde(rename = "type")]
    pub type_: Option<u32>,
    /// The resolved data of the interaction.
    pub resolved: Option<ResolvedData>,
    /// The options of the interaction.
    pub options: Option<Vec<AppCommandInteractionData>>,
    /// The guild ID where the interaction was sent.
    pub guild_id: Option<String>,
    /// The target ID of the interaction.
    pub target_id: Option<String>,
}

/// Resolved data associated with an interaction.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct ResolvedData {
    /// The users involved in the interaction.
    pub users: Option<HashMap<String, User>>,
    /// The members involved in the interaction.
    pub members: Option<HashMap<String, Member>>,
    /// The channels involved in the interaction.
    pub channels: Option<HashMap<String, Channel>>,
    /// The messages involved in the interaction.
    pub messages: Option<HashMap<String, Message>>,
    // TODO: roles, attachments
}

/// Data for an application command interaction.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct AppCommandInteractionData {
    /// The name of the command.
    pub name: String,
    /// The type of the command.
    #[nserde(rename = "type")]
    pub type_: u32,
    /// The value of the command.
    /// Can be a string, int, float, or bool.
    pub value: String,
    /// The options for the command.
    pub options: Option<Vec<AppCommandInteractionData>>,
    /// Whether the command is focused.
    pub focused: Option<bool>,
}

/// Choices for an autocomplete interaction.
#[derive(DeJson, SerJson, Clone, Debug, Default)]
pub struct InteractionAutoCompleteChoices {
    /// The type of the interaction.
    #[nserde(rename = "type")]
    pub type_: u32,
    /// The data for the interaction.
    pub data: Option<InteractionAutoCompleteChoicePlaceholder>,
}

impl InteractionAutoCompleteChoices {
    /// Creates a new instance of `InteractionAutoCompleteChoices`.
    ///
    /// # Arguments
    ///
    /// * `choices` - The autocomplete choices.
    ///
    /// # Examples
    ///
    /// ```
    /// let choices = InteractionAutoCompleteChoices::new(vec![choice1, choice2]);
    /// ```
    pub fn new(choices: Vec<InteractionAutoCompleteChoice>) -> Self {
        Self {
            type_: InteractionCallbackType::ApplicationCommandAutocompleteResult as _,
            data: Some(InteractionAutoCompleteChoicePlaceholder { choices }),
        }
    }
}

/// Placeholder for autocomplete choices.
#[derive(DeJson, SerJson, Clone, Debug, Default)]
pub struct InteractionAutoCompleteChoicePlaceholder {
    /// The autocomplete choices.
    pub choices: Vec<InteractionAutoCompleteChoice>,
}

/// Represents an autocomplete choice.
#[derive(DeJson, SerJson, Clone, Debug, Default)]
pub struct InteractionAutoCompleteChoice {
    /// The name of the choice.
    pub name: String,
    /// The value of the choice.
    pub value: String,
}

/// Represents an interaction response.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct InteractionResponse {
    /// The type of the interaction response.
    #[nserde(rename = "type")]
    pub type_: u32,
    /// The data for the interaction response.
    pub data: Option<CreateMessageData>,
}
