use super::message_response::CreateMessageData;
use super::{channel::Channel, user::User};
use crate::consts::DISCORD_CDN;
use crate::prelude::Role;
use crate::{prelude::ImageFormat, utils};
use nanoserde::{DeJson, SerJson};
use reqwest::Method;

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Guild {
    pub id: String,
    pub name: String,
    #[nserde(rename = "icon")]
    pub icon_hash: Option<String>,
    #[nserde(rename = "splash")]
    pub splash_hash: Option<String>,
    #[nserde(rename = "discovery_splash")]
    pub discovery_splash_hash: Option<String>,
    pub owner_id: String,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: usize,
    #[nserde(default)]
    pub widget_enabled: bool,
    pub widget_channel_id: Option<String>,
    pub verification_level: usize,
    #[nserde(rename = "default_message_notifications")]
    pub default_message_notifications_level: usize,
    #[nserde(rename = "explicit_content_filter")]
    pub explicit_content_filter_level: usize,
    pub mfa_level: usize,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flag: Option<usize>,
    pub rules_channel_id: Option<String>,
    #[nserde(default)]
    pub max_members: Option<usize>,
    pub vanity_url_code: Option<String>,
    #[nserde(rename = "banner")]
    pub banner_hash: Option<String>,
    #[nserde(default, rename = "premium_subscription_count")]
    pub boost_count: Option<u32>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<usize>,
    pub nsfw_level: usize,
    #[nserde(rename = "premium_progress_bar_enabled")]
    pub boost_progress_bar: bool,
    pub safety_alerts_channel_id: Option<String>,
    // TODO: permissions, roles, welcome_screen, sticker
}

#[derive(DeJson, SerJson)]
pub struct GuildCreateResponse {
    #[nserde(rename = "d")]
    pub data: GuildCreate,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct GuildCreate {
    pub id: String,
    pub name: String,
    #[nserde(rename = "icon")]
    pub icon_hash: Option<String>,
    #[nserde(rename = "splash")]
    pub splash_hash: Option<String>,
    #[nserde(rename = "discovery_splash")]
    pub discovery_splash_hash: Option<String>,
    pub owner_id: String,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: usize,
    #[nserde(default)]
    pub widget_enabled: bool,
    pub widget_channel_id: Option<String>,
    pub verification_level: usize,
    #[nserde(rename = "default_message_notifications")]
    pub default_message_notifications_level: usize,
    #[nserde(rename = "explicit_content_filter")]
    pub explicit_content_filter_level: usize,
    pub mfa_level: usize,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flag: Option<usize>,
    pub rules_channel_id: Option<String>,
    #[nserde(default)]
    pub max_members: usize,
    pub vanity_url_code: Option<String>,
    #[nserde(rename = "banner")]
    pub banner_hash: Option<String>,
    #[nserde(default, rename = "premium_subscription_count")]
    pub boost_count: Option<u32>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<usize>,
    pub nsfw_level: usize,
    #[nserde(rename = "premium_progress_bar_enabled")]
    pub boost_progress_bar: bool,
    pub safety_alerts_channel_id: Option<String>,

    // Extra fields
    pub joined_at: String,
    pub large: bool,
    pub unavailable: Option<bool>,
    pub member_count: usize,
    pub members: Vec<Member>,
    pub channels: Vec<Channel>,
    pub threads: Vec<Channel>,
}

#[doc(alias = "UnavailableGuild")]
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct PartialGuild {
    pub unavailable: bool,
    pub id: String,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    #[nserde(rename = "avatar")]
    pub guild_avatar_hash: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    #[nserde(default)]
    pub deaf: bool,
    #[nserde(default)]
    pub mute: bool,
    pub flags: usize,
    #[nserde(default)]
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    #[nserde(rename = "communication_disabled_until")]
    pub timeout_until: Option<String>,
    // #[nserde(rename = "avatar_decoration_data")]
    // pub avatar_decoration: Option<AvatarDecorationData>,
    #[nserde(default)]
    pub mention: String,
}

impl Guild {
    pub async fn fetch_member(&self, user_id: &str) -> Result<Member, Box<dyn std::error::Error>> {
        utils::fetch_member(&self.id, user_id).await
    }

    pub async fn fetch_role(&self, role_id: &str) -> Result<Role, Box<dyn std::error::Error>> {
        utils::fetch_role(&self.id, role_id).await
    }

    pub async fn default_role(&self) -> Result<Role, Box<dyn std::error::Error>> {
        utils::fetch_role(&self.id, &self.id).await
    }
}

impl Member {
    pub fn get_avatar_url(&self, image_format: ImageFormat, size: Option<u32>) -> Option<String> {
        let size = if let Some(size) = size {
            if !size.is_power_of_two() {
                log::error!("size must be powers of 2");
            }

            format!("?size={size}")
        } else {
            "".to_string()
        };

        let user = self.user.as_ref().unwrap();
        let user_id = &user.id;
        let avatar_hash = user.avatar_hash.as_ref()?;

        Some(format!(
            "{DISCORD_CDN}/avatars/{user_id}/{avatar_hash}{image_format}{size}"
        ))
    }

    pub async fn send_dm(&self, data: impl Into<CreateMessageData>) {
        utils::send_dm(&self.user.as_ref().unwrap().id, data).await;
    }
}
