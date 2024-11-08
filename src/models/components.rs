use nanoserde::{DeJson, SerJson};

use crate::consts::{ButtonStyle, ChannelType, SelectMenuType};

use super::emoji::Emoji;

/// Represents a component in a Discord message.
#[derive(DeJson, SerJson, Debug, Default, Clone)]
pub struct Component {
    /// Type of component.
    #[nserde(default = 1, rename = "type")]
    pub type_: u32,
    /// Nested components.
    pub components: Option<Vec<Component>>,
    /// Button style.
    pub style: Option<u32>,
    /// Button label.
    pub label: Option<String>,
    /// Button emoji.
    pub emoji: Option<Emoji>,
    /// Custom ID for the component.
    pub custom_id: Option<String>,
    /// URL for the button.
    pub url: Option<String>,
    /// Whether the component is disabled.
    #[nserde(default)]
    pub disabled: bool,
    /// Options for select menus.
    pub options: Option<Vec<SelectOption>>,
    /// Channel types for select menus.
    pub channel_types: Option<Vec<u32>>,
    /// Placeholder text for select menus.
    pub placeholder: Option<String>,
    /// Default values for select menus.
    pub default_values: Option<Vec<SelectDefaultValue>>,
    /// Minimum values for select menus.
    ///
    /// # TODO
    /// Add checks for the range: 0-25.
    pub min_values: Option<u32>,
    /// Maximum values for select menus.
    ///
    /// Defaults to 1.
    ///
    /// # TODO
    /// Add checks for the range: 0-25.
    pub max_values: Option<u32>,
}

/// Represents an option in a select menu.
#[derive(DeJson, SerJson, Debug, Default, Clone)]
pub struct SelectOption {
    /// Label for the option.
    pub label: String,
    /// Value for the option.
    pub value: String,
    /// Description for the option.
    pub description: Option<String>,
    /// Emoji for the option.
    pub emoji: Option<Emoji>,
    /// Whether the option is the default.
    pub default: Option<bool>,
}

/// Represents a default value in a select menu.
#[derive(DeJson, SerJson, Debug, Default, Clone)]
pub struct SelectDefaultValue {
    /// ID of a user, role, or channel.
    pub id: String,
    /// Either "user", "roles", or "channel".
    #[nserde(rename = "type")]
    pub type_: String,
}

/// Represents a select menu object.
#[derive(Debug, Clone, Default)]
pub struct SelectObject {
    /// Type of the select menu.
    pub select_type: SelectMenuType,
    /// Custom ID for the select menu.
    pub custom_id: String,
    /// Options for the select menu.
    pub options: Option<Vec<SelectOption>>,
    /// Channel types for the select menu.
    pub channel_types: Option<Vec<ChannelType>>,
    /// Placeholder text for the select menu.
    pub placeholder: Option<String>,
    /// Default values for the select menu.
    pub default_values: Option<Vec<SelectDefaultValue>>,
    /// Minimum values for the select menu.
    pub min_values: Option<u32>,
    /// Maximum values for the select menu.
    pub max_values: Option<u32>,
    /// Whether the select menu is disabled.
    pub disabled: bool,
}

impl SelectObject {
    /// Verifies the select menu object.
    ///
    /// # Errors
    ///
    /// Returns an error if the min or max values are out of range.
    pub(crate) fn verify(&self) -> Result<(), &'static str> {
        if self.min_values.map(|i| i <= 25) == Some(false)
            || self.max_values.map(|i| i <= 25) == Some(false)
        {
            return Err("Min and max values should be in the range 0 to 25");
        }

        Ok(())
    }
}

/// Represents a button object.
#[derive(Default)]
pub struct ButtonObject {
    /// Style of the button.
    pub style: u32,
    /// Label for the button.
    pub label: Option<String>,
    /// Emoji for the button.
    pub emoji: Option<Emoji>,
    /// Custom ID for the button.
    pub custom_id: Option<String>,
    /// URL for the button.
    pub url: Option<String>,
    /// Whether the button is disabled.
    pub disabled: bool,
}

impl ButtonObject {
    /// Verifies the button object.
    ///
    /// # Errors
    ///
    /// Returns an error if the button configuration is invalid.
    pub(crate) fn verify(&self) -> Result<(), &'static str> {
        if self.style == ButtonStyle::Link as u32 {
            if self.custom_id.is_some() {
                return Err("Link buttons cannot have a custom id");
            }

            if self.url.is_none() {
                return Err("Link buttons must have a url");
            }
        }

        Ok(())
    }
}
