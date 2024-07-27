use json::JsonValue;
use std::collections::HashMap;

use crate::consts::*;
use crate::models::allowed_mentions::AllowedMentions;
use crate::models::guild::Member;
use crate::prelude::{Component, Embed};
use crate::utils::request;
use nanoserde::{DeJson, SerJson};
use reqwest::Method;

use super::message_response::CreateMessageData;
use super::{channel::Channel, message_response::Message, user::User};

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct InteractionResponsePayload {
    #[nserde(rename = "d")]
    pub data: Interaction,
}

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    #[nserde(rename = "type")]
    pub type_: u32,
    pub data: Option<InteractionData>,
    pub channel: Option<Channel>,
    pub channel_id: Option<String>,
    pub member: Option<Member>,
    pub token: String,
    pub message: Option<Message>,
    pub app_permissions: String,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
    pub context: Option<u32>,

    /// User object for the invoking user, if invoked in a DM
    pub user: Option<User>,
}

impl Interaction {
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

#[derive(DeJson, SerJson, Clone, Debug, Default)]
pub struct InteractionData {
    pub custom_id: Option<String>,
    pub component_type: Option<u32>,

    pub id: Option<String>,
    #[nserde(rename = "name")]
    pub command_name: Option<String>,
    #[nserde(rename = "type")]
    pub type_: Option<u32>,
    pub resolved: Option<ResolvedData>,
    pub options: Option<Vec<AppCommandInteractionData>>,
    pub guild_id: Option<String>,
    pub target_id: Option<String>,
}

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct ResolvedData {
    pub users: Option<HashMap<String, User>>,
    pub members: Option<HashMap<String, Member>>,
    pub channels: Option<HashMap<String, Channel>>,
    pub messages: Option<HashMap<String, Message>>,
    // TODO: roles, attachments
}

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct AppCommandInteractionData {
    pub name: String,
    #[nserde(rename = "type")]
    pub type_: u32,
    /// string, int, float, or bool
    pub value: String,
    pub options: Option<Vec<AppCommandInteractionData>>,
    pub focused: Option<bool>,
}

#[derive(DeJson, SerJson, Clone, Debug, Default)]
pub struct InteractionAutoCompleteChoices {
    #[nserde(rename = "type")]
    pub type_: u32,
    pub data: Option<InteractionAutoCompleteChoicePlaceholder>,
}

impl InteractionAutoCompleteChoices {
    pub fn new(choices: Vec<InteractionAutoCompleteChoice>) -> Self {
        Self {
            type_: InteractionCallbackType::ApplicationCommandAutocompleteResult as _,
            data: Some(InteractionAutoCompleteChoicePlaceholder { choices }),
        }
    }
}

#[derive(DeJson, SerJson, Clone, Debug, Default)]
pub struct InteractionAutoCompleteChoicePlaceholder {
    pub choices: Vec<InteractionAutoCompleteChoice>,
}

#[derive(DeJson, SerJson, Clone, Debug, Default)]
pub struct InteractionAutoCompleteChoice {
    pub name: String,
    pub value: String,
}

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct InteractionResponse {
    #[nserde(rename = "type")]
    pub type_: u32,
    pub data: Option<CreateMessageData>,
}
