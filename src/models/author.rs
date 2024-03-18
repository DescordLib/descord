use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Clone)]
pub struct Author {
    pub username: String,
    #[nserde(rename = "id")]
    pub user_id: String,
    pub global_name: Option<String>,
    pub discriminator: String,
    pub application_id: Option<String>,

    #[nserde(rename = "avatar")]
    pub avatar_hash: Option<String>,

    #[nserde(default)]
    pub bot: bool,
    // TODO: attachments
}

impl Author {
    pub fn get_avatar_url(&self) -> Option<String> {
        let user_id = &self.user_id;
        let avatar_hash = self.avatar_hash.as_ref()?;

        Some(format!(
            "https://cdn.discordapp.com/avatars/{user_id}/{avatar_hash}.webp"
        ))
    }
}
