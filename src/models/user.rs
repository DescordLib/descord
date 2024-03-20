use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug)]
pub struct User {
    pub verified: bool,
    pub username: String,
    pub mfa_enabled: bool,
    pub id: String,
    pub global_name: Option<String>,
    pub flags: usize,
    pub email: Option<String>,
    pub discriminator: String,
    pub bot: bool,
    pub avatar: Option<String>,
}
