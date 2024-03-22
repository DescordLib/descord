use nanoserde::{DeJson, SerJson};

use crate::{utils, consts::ImageFormat};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct User {
    #[nserde(default)]
    pub verified: bool,
    #[nserde(default)]
    pub username: String,
    #[nserde(default)]
    pub mfa_enabled: bool,
    #[nserde(default)]
    pub id: String,
    #[nserde(default)]
    pub global_name: Option<String>,
    #[nserde(default)]
    pub flags: usize,
    #[nserde(default)]
    pub email: Option<String>,
    #[nserde(default)]
    pub discriminator: String,
    #[nserde(default)]
    pub bot: bool,
    #[nserde(default)]
    pub avatar: Option<String>,
}

impl User {
    pub fn get_avatar_url(&self, image_format: ImageFormat) -> Option<String> {
        let user_id = &self.id;
        let avatar_hash = self.avatar.as_ref()?;

        Some(format!(
            "https://cdn.discordapp.com/avatars/{user_id}/{avatar_hash}{image_format}"
        ))
    }

    pub fn mention(&self) -> String {
        format!("<@{0}>", self.id)
    }
}

