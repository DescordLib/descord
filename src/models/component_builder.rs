use crate::consts::ButtonStyle;

use super::{
    components::{ButtonObject, Component},
    emoji::Emoji,
};

pub struct ComponentBuilder;

impl ComponentBuilder {
    pub fn button(button: ButtonObject) -> Result<Component, &'static str> {
        button.verify()?;
        Ok(button.into())
    }
}
