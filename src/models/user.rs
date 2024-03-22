use nanoserde::{DeJson, SerJson};

use crate::{consts::ImageFormat, utils};

use crate::consts::*;

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
    pub global_name: Option<String>,
    #[nserde(default)]
    pub flags: usize,
    pub email: Option<String>,
    #[nserde(default)]
    pub discriminator: String,
    #[nserde(default)]
    pub bot: bool,
    #[nserde(rename = "avatar")]
    pub avatar_hash: Option<String>,
}

impl User {
    pub fn get_avatar_url(&self, image_format: ImageFormat, size: Option<u32>) -> Option<String> {
        let size = if let Some(size) = size {
            if !size.is_power_of_two() {
                log::error!("size must be powers of 2");
            }

            format!("?size={size}")
        } else {
            "".to_string()
        };

        let user_id = &self.id;
        let avatar_hash = self.avatar_hash.as_ref()?;

        Some(format!(
            "{DISCORD_CDN}/avatars/{user_id}/{avatar_hash}{image_format}{size}"
        ))
    }

    pub fn mention(&self) -> String {
        format!("<@{0}>", self.id)
    }
}
