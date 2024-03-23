use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct AllowedMentions {
    pub parse: Option<Vec<String>>,
    pub roles: Option<Vec<String>>,
    pub users: Option<Vec<String>>,
    pub replied_user: Option<bool>,
}
