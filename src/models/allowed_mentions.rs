use nanoserde::{DeJson, SerJson};

/// Represents allowed mentions in a message.
///
/// This struct is used to specify which mentions should be parsed in a message.
/// It can be used to prevent certain mentions from being parsed.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct AllowedMentions {
    /// An array of mention types to parse from the content.
    /// Valid values are "roles", "users", and "everyone".
    pub parse: Option<Vec<String>>,
    /// An array of role IDs to mention (max size of 100).
    pub roles: Option<Vec<String>>,
    /// An array of user IDs to mention (max size of 100).
    pub users: Option<Vec<String>>,
    /// For replies, whether to mention the author of the message being replied to.
    pub replied_user: Option<bool>,
}
