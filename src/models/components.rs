use nanoserde::{DeJson, SerJson};

use crate::consts::{ButtonStyle, ChannelType, SelectMenuType};

use super::emoji::Emoji;

#[derive(DeJson, SerJson, Debug, Default, Clone)]
pub struct Component {
    /// Type of component
    #[nserde(default = 1, rename = "type")]
    pub type_: u32,
    pub components: Option<Vec<Component>>,
    /// Button style
    pub style: Option<u32>,
    pub label: Option<String>,
    pub emoji: Option<Emoji>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    #[nserde(default)]
    pub disabled: bool,
    pub options: Option<Vec<SelectOption>>,
    pub channel_types: Option<Vec<u32>>,
    pub placeholder: Option<String>,
    pub default_values: Option<Vec<SelectDefaultValue>>,

    // TODO: add checks for the range: 0-25
    pub min_values: Option<u32>,

    // TODO: add checks for the range: 0-25
    /// Defaults to 1
    pub max_values: Option<u32>,
}

#[derive(DeJson, SerJson, Debug, Default, Clone)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}

#[derive(DeJson, SerJson, Debug, Default, Clone)]
pub struct SelectDefaultValue {
    /// ID of a user, role, or channel
    pub id: String,

    /// Either "user", "roles", or "channel"
    #[nserde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Default)]
pub struct SelectObject {
    pub select_type: SelectMenuType,
    pub custom_id: String,
    pub options: Option<Vec<SelectOption>>,
    pub channel_types: Option<Vec<ChannelType>>,
    pub placeholder: Option<String>,
    pub default_values: Option<Vec<SelectDefaultValue>>,
    pub min_values: Option<u32>,
    pub max_values: Option<u32>,
    pub disabled: bool,
}

impl SelectObject {
    pub(crate) fn verify(&self) -> Result<(), &'static str> {
        if self.min_values.map(|i| i <= 25) == Some(false)
            || self.max_values.map(|i| i <= 25) == Some(false)
        {
            return Err("Min and max values should be in the range 0 to 25");
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct ButtonObject {
    pub style: u32,
    pub label: Option<String>,
    pub emoji: Option<Emoji>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    pub disabled: bool,
}

impl ButtonObject {
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
