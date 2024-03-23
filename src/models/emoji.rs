use nanoserde::{DeJson, SerJson};

use super::user::User;

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Emoji {
    pub name: String,
    pub id: Option<String>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

impl Emoji {
    pub fn parse(emoji: &str) -> Self {
        let emoji = emoji.trim_matches(['<', '>', ':']);
        let name: String;
        let id = if let Some((name_, id)) = emoji.split_once(':') {
            // <:name:1234> -> name, 1234
            name = name_.to_owned();
            Some(id.to_string())
        } else {
            // :star: -> star
            name = emoji.to_string();
            None
        };

        Self {
            id,
            name,
            ..Default::default()
        }
    }
}
