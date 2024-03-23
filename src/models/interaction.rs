use json::JsonValue;
use std::collections::HashMap;

use crate::consts::*;
use crate::models::allowed_mentions::AllowedMentions;
use crate::models::guild::Member;
use crate::prelude::{Component, Embed};
use crate::utils::send_request;
use nanoserde::{DeJson, SerJson};
use reqwest::Method;

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
    pub user: Option<User>,
    pub token: String,
    pub message: Option<Message>,
    pub app_permissions: String,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
    pub context: Option<u32>,
}

impl Interaction {
    pub async fn reply(&self, response: &str) -> Result<reqwest::Response, reqwest::Error> {
        let response = InteractionResponse {
            type_: 4,
            data: Some(InteractionResponseData {
                content: Some(response.to_string()),
                ..Default::default()
            }),
        };
        let json_response = SerJson::serialize_json(&response);
        send_request(
            Method::POST,
            format!("interactions/{}/{}/callback", self.id, self.token).as_str(),
            Some(JsonValue::from(json_response)),
        )
        .await
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

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct InteractionResponse {
    #[nserde(rename = "type")]
    pub type_: u32,
    pub data: Option<InteractionResponseData>,
}

#[derive(DeJson, SerJson, Clone, Debug, Default)]
pub struct InteractionResponseData {
    pub tts: Option<bool>,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub allowed_mentions: Option<AllowedMentions>,
    pub flags: Option<u32>,
    pub components: Option<Vec<Component>>,
}
