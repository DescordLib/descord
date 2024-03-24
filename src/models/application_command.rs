use std::collections::HashMap;

use nanoserde::{DeJson, SerJson};

#[derive(Debug, Clone, DeJson, SerJson)]
pub struct ApplicationCommand {
    pub id: String,
    #[nserde(rename = "type")]
    pub type_: Option<u32>,
    pub application_id: String,
    pub guild_id: Option<String>,
    pub name: String,
    // name_localizations: Option<>,
    pub description: String,
    // description_localizations: Option<>,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub default_member_permissions: Option<String>,
    pub nsfw: Option<bool>,
    pub integration_types: Vec<u32>,
    pub contexts: Option<Vec<u32>>,
    pub version: String,
}

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct ApplicationCommandOption {
    #[nserde(rename = "type")]
    pub type_: u32,
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,
    pub required: Option<bool>,
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub channel_types: Option<Vec<u32>>,
    pub min_value: Option<i32>,
    pub max_value: Option<i32>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub autocomplete: Option<bool>,
}

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: String,
}
