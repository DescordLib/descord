use nanoserde::{SerJson, DeJson};

#[derive(DeJson, SerJson, Clone)]
pub struct Author {
    pub username: String,
    #[nserde(rename = "id")]
    pub user_id: String,
    pub global_name: Option<String>,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub application_id: Option<String>,

    #[nserde(default)]
    pub bot: bool,

    // TODO: attachments
}
