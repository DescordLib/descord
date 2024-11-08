use crate::{consts::ImageFormat, utils};
use nanoserde::{DeJson, SerJson};

use crate::consts::*;

/// Represents a user in Discord.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct User {
    /// Whether the user is verified.
    #[nserde(default)]
    pub verified: bool,
    /// The username of the user.
    #[nserde(default)]
    pub username: String,
    /// Whether the user has multi-factor authentication enabled.
    #[nserde(default)]
    pub mfa_enabled: bool,
    /// The unique ID of the user.
    #[nserde(default)]
    pub id: String,
    /// The global name of the user.
    pub global_name: Option<String>,
    /// The flags of the user.
    #[nserde(default)]
    pub flags: usize,
    /// The email of the user.
    pub email: Option<String>,
    /// The discriminator of the user.
    #[nserde(default)]
    pub discriminator: String,
    /// Whether the user is a bot.
    #[nserde(default)]
    pub bot: bool,
    /// The hash of the user's avatar.
    #[nserde(rename = "avatar")]
    pub avatar_hash: Option<String>,
    /// The mention string for the user.
    #[nserde(default)]
    pub mention: String,
}

impl User {
    /// Gets the avatar URL of the user.
    ///
    /// # Arguments
    ///
    /// * `image_format` - The format of the image.
    /// * `size` - The size of the image.
    ///
    /// # Examples
    ///
    /// ```
    /// let avatar_url = user.get_avatar_url(ImageFormat::PNG, Some(128));
    /// ```
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
}
