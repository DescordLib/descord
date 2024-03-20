use nanoserde::{DeJson, SerJson};

use super::user::User;

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Guild {
    pub unavailable: bool,
    pub id: String,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct GuildMember {
    pub user: User,
    pub roles: Vec<String>,
    pub mute: bool,
    pub pending: bool,
    pub joined_at: String,

    #[nserde(default)]
    pub flags: usize,
    #[nserde(default)]
    pub premium_since: Option<String>,
    #[nserde(default)]
    pub nick: Option<String>,

    // TODO
    // avatar:
}
