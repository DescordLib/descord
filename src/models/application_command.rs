use std::collections::HashMap;

use nanoserde::{DeJson, SerJson};

/// Represents an application command.
#[derive(Debug, Clone, DeJson, SerJson)]
pub struct ApplicationCommand {
    /// The unique ID of the command.
    pub id: String,
    /// The type of the command.
    #[nserde(rename = "type")]
    pub type_: Option<u32>,
    /// The application ID of the command.
    pub application_id: String,
    /// The guild ID where the command is registered.
    pub guild_id: Option<String>,
    /// The name of the command.
    pub name: String,
    // name_localizations: Option<>,
    /// The description of the command.
    pub description: String,
    // description_localizations: Option<>,
    /// The options for the command.
    pub options: Option<Vec<ApplicationCommandOption>>,
    /// The default member permissions required to use the command.
    pub default_member_permissions: Option<String>,
    /// Whether the command is marked as NSFW.
    pub nsfw: Option<bool>,
    /// The integration types for the command.
    pub integration_types: Vec<u32>,
    /// The contexts in which the command can be used.
    pub contexts: Option<Vec<u32>>,
    /// The version of the command.
    pub version: String,
}

/// Represents an option for an application command.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct ApplicationCommandOption {
    /// The type of the option.
    #[nserde(rename = "type")]
    pub type_: u32,
    /// The name of the option.
    pub name: String,
    /// Localized names for the option.
    pub name_localizations: Option<HashMap<String, String>>,
    /// The description of the option.
    pub description: String,
    /// Localized descriptions for the option.
    pub description_localizations: Option<HashMap<String, String>>,
    /// Whether the option is required.
    pub required: Option<bool>,
    /// The choices for the option.
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    /// The sub-options for the option.
    pub options: Option<Vec<ApplicationCommandOption>>,
    /// The channel types for the option.
    pub channel_types: Option<Vec<u32>>,
    /// The minimum value for the option.
    pub min_value: Option<i32>,
    /// The maximum value for the option.
    pub max_value: Option<i32>,
    /// The minimum length for the option.
    pub min_length: Option<i32>,
    /// The maximum length for the option.
    pub max_length: Option<i32>,
    /// Whether the option supports autocomplete.
    pub autocomplete: Option<bool>,
}

/// Represents a choice for an application command option.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct ApplicationCommandOptionChoice {
    /// The name of the choice.
    pub name: String,
    /// Localized names for the choice.
    pub name_localizations: Option<HashMap<String, String>>,
    /// The value of the choice.
    pub value: String,
}
