use nanoserde::{DeJson, SerJson};

use crate::consts::ButtonStyle;

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
    label: String,
    value: String,
    description: Option<String>,
    emoji: Option<Emoji>,
    default: Option<bool>,
}

#[derive(DeJson, SerJson, Debug, Default, Clone)]
pub struct SelectDefaultValue {
    pub id: String,

    /// Either "user", "roles", or "channel"
    #[nserde(rename = "type")]
    pub type_: String,
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
    pub fn verify(&self) -> Result<(), &'static str> {
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

impl Into<Component> for ButtonObject {
    fn into(self) -> Component {
        let ButtonObject {
            style,
            label,
            emoji,
            custom_id,
            url,
            disabled,
        } = self;

        Component {
            type_: 2,
            label,
            emoji,
            custom_id,
            url,
            disabled,
            style: Some(style),
            components: None,
            options: None,
            channel_types: None,
            placeholder: None,
            default_values: None,
            min_values: None,
            max_values: None,
        }
    }
}
