use nanoserde::{DeJson, SerJson};

use super::user::User;

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Emoji {
    pub name: String,
    pub id: Option<String>,
    #[nserde(default)]
    pub user: Option<User>,
    #[nserde(default)]
    pub require_colons: Option<bool>,
    #[nserde(default)]
    pub managed: Option<bool>,
    #[nserde(default)]
    pub animated: Option<bool>,
    #[nserde(default)]
    pub available: Option<bool>,
}
