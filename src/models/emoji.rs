use nanoserde::{DeJson, SerJson};

use super::user::User;

/// Represents an emoji in Discord.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Emoji {
    /// The name of the emoji.
    pub name: String,
    /// The unique ID of the emoji.
    pub id: Option<String>,
    /// The user who created the emoji.
    pub user: Option<User>,
    /// Whether the emoji requires colons to be used.
    pub require_colons: Option<bool>,
    /// Whether the emoji is managed.
    pub managed: Option<bool>,
    /// Whether the emoji is animated.
    pub animated: Option<bool>,
    /// Whether the emoji is available.
    pub available: Option<bool>,
}

impl Emoji {
    /// Parses a string into an `Emoji` object.
    ///
    /// # Arguments
    ///
    /// * `emoji` - The string representation of the emoji.
    ///
    /// # Examples
    ///
    /// ```
    /// let emoji = Emoji::parse(":star:");
    /// ```
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
