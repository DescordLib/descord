use std::collections::HashMap;

use crate::consts::*;
use crate::models::guild::GuildMember;
use crate::models::interaction::Interaction;
use crate::prelude::{Component, Embed};
use nanoserde::{DeJson, SerJson};

use super::{channel::Channel, message_response::Message, user::User};

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct AllowedMentions {
    pub parse: Option<Vec<String>>,
    pub roles: Option<Vec<String>>,
    pub users: Option<Vec<String>>,
    pub replied_user: Option<bool>,
}
