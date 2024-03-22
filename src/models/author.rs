use nanoserde::{DeJson, SerJson};

use crate::{consts::ImageFormat, utils};

use super::message_response::CreateMessageData;
use crate::consts::*;

#[derive(DeJson, SerJson, Clone, Debug)]
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
    pub fn get_avatar_url(&self, image_format: ImageFormat, size: Option<u32>) -> Option<String> {
        let size = if let Some(size) = size {
            if !size.is_power_of_two() {
                log::error!("size must be powers of 2");
            }

            format!("?size={size}")
        } else {
            "".to_string()
        };

        let user_id = &self.user_id;
        let avatar_hash = self.avatar_hash.as_ref()?;

        Some(format!(
            "{DISCORD_CDN}/avatars/{user_id}/{avatar_hash}{image_format}{size}"
        ))
    }

    pub async fn send_dm(&self, data: impl Into<CreateMessageData>) {
        utils::send_dm(&self.user_id, data).await;
    }
}
