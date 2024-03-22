use nanoserde::{DeJson, SerJson};
use crate::consts::*;
use crate::prelude::User;

#[derive(SerJson, Clone, Debug)]
pub struct Member {
    pub user: User,
    pub roles: Vec<String>,
    pub premium_since: Option<String>,
    pub permissions: String,
    pub pending: bool,
    pub nick: Option<String>,
    pub mute: bool,
    pub joined_at: String,
    pub is_pending: bool,
    pub deaf: bool,
}