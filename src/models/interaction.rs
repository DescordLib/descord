use std::collections::HashMap;

use crate::consts::*;
use crate::models::guild::GuildMember;
use nanoserde::{DeJson, SerJson};

use super::{channel::Channel, user::User, message_response::Message};

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    #[nserde(rename = "type")]
    pub type_: u32,
    pub data: Option<InteractionData>,
    pub channel: Option<Channel>,
    pub channel_id: Option<String>,
    pub member: Option<GuildMember>,
    pub user: Option<User>,
    pub token: String,
    pub message: Option<Message>,
    pub app_permissions: String,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
    pub context: Option<u32>,
}

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct InteractionData {
    pub id: String,
    #[nserde(rename = "name")]
    pub command_name: String,
    #[nserde(rename = "type")]
    pub type_: u32,
    pub resolved: Option<ResolvedData>,
    pub options: Option<Vec<AppCommandInteractionData>>,
    pub guild_id: Option<String>,
    pub target_id: Option<String>,
}

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct ResolvedData {
    pub users: Option<HashMap<String, User>>,
    pub members: Option<HashMap<String, GuildMember>>,
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
