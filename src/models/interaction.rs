use nanoserde::{DeJson, SerJson};
use crate::consts::*;
use crate::models::member::Member;

#[derive( SerJson, Clone, Debug)]
pub struct Interaction {
    pub r#type: i32,
    pub token: String,
    pub member: Member,
    pub id: String,
    pub guild_id: String,
    pub app_permissions: String,
    pub guild_locale: String,
    pub locale: String,
    pub data: InteractionData,
    pub channel_id: String,
}

impl Interaction {
    pub fn get_name(&self) -> &str {
        &self.data.name
    }

    pub fn get_options(&self) -> &Vec<InteractionOption> {
        &self.data.options
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_channel_id(&self) -> &str {
        &self.channel_id
    }
}

#[derive(SerJson, Clone, Debug)]
pub struct InteractionData {
    pub name: String,
    pub options: Vec<InteractionOption>,
    pub r#type: i32,
    pub id: String,
}

#[derive( SerJson, Clone, Debug)]
pub struct InteractionOption {
    pub name: String,
    pub value: String,
    pub r#type: i32,
}