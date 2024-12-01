use super::message_response::CreateMessageData;
use super::{channel::Channel, user::User};
use crate::consts::DISCORD_CDN;
use crate::prelude::Role;
use crate::{prelude::ImageFormat, utils};
use nanoserde::{DeJson, SerJson};
use reqwest::Method;

/// Represents a Discord guild (server).
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Guild {
    /// The unique ID of the guild.
    pub id: String,
    /// The name of the guild.
    pub name: String,
    /// The hash of the guild icon.
    #[nserde(rename = "icon")]
    pub icon_hash: Option<String>,
    /// The hash of the guild splash image.
    #[nserde(rename = "splash")]
    pub splash_hash: Option<String>,
    /// The hash of the guild discovery splash image.
    #[nserde(rename = "discovery_splash")]
    pub discovery_splash_hash: Option<String>,
    /// The ID of the guild owner.
    pub owner_id: String,
    /// The ID of the AFK channel.
    pub afk_channel_id: Option<String>,
    /// The AFK timeout in seconds.
    pub afk_timeout: usize,
    /// Whether the widget is enabled.
    #[nserde(default)]
    pub widget_enabled: bool,
    /// The ID of the widget channel.
    pub widget_channel_id: Option<String>,
    /// The verification level of the guild.
    pub verification_level: usize,
    /// The default message notifications level.
    #[nserde(rename = "default_message_notifications")]
    pub default_message_notifications_level: usize,
    /// The explicit content filter level.
    #[nserde(rename = "explicit_content_filter")]
    pub explicit_content_filter_level: usize,
    /// The MFA level required for the guild.
    pub mfa_level: usize,
    /// The application ID of the guild.
    pub application_id: Option<String>,
    /// The ID of the system channel.
    pub system_channel_id: Option<String>,
    /// The system channel flags.
    pub system_channel_flag: Option<usize>,
    /// The ID of the rules channel.
    pub rules_channel_id: Option<String>,
    /// The maximum number of members in the guild.
    #[nserde(default)]
    pub max_members: Option<usize>,
    /// The vanity URL code of the guild.
    pub vanity_url_code: Option<String>,
    /// The hash of the guild banner.
    #[nserde(rename = "banner")]
    pub banner_hash: Option<String>,
    /// The number of boosts the guild has.
    #[nserde(default, rename = "premium_subscription_count")]
    pub boost_count: Option<u32>,
    /// The preferred locale of the guild.
    pub preferred_locale: String,
    /// The ID of the public updates channel.
    pub public_updates_channel_id: Option<String>,
    /// The maximum number of users in a video channel.
    pub max_video_channel_users: Option<usize>,
    /// The NSFW level of the guild.
    pub nsfw_level: usize,
    /// Whether the boost progress bar is enabled.
    #[nserde(rename = "premium_progress_bar_enabled")]
    pub boost_progress_bar: bool,
    /// The ID of the safety alerts channel.
    pub safety_alerts_channel_id: Option<String>,
    // TODO: permissions, roles, welcome_screen, sticker
}

/// Represents the response for a guild creation.
#[derive(DeJson, SerJson)]
pub struct GuildCreateResponse {
    /// The guild creation data.
    #[nserde(rename = "d")]
    pub data: GuildCreate,
}

/// Represents the data for a guild creation.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct GuildCreate {
    /// The unique ID of the guild.
    pub id: String,
    /// The name of the guild.
    pub name: String,
    /// The hash of the guild icon.
    #[nserde(rename = "icon")]
    pub icon_hash: Option<String>,
    /// The hash of the guild splash image.
    #[nserde(rename = "splash")]
    pub splash_hash: Option<String>,
    /// The hash of the guild discovery splash image.
    #[nserde(rename = "discovery_splash")]
    pub discovery_splash_hash: Option<String>,
    /// The ID of the guild owner.
    pub owner_id: String,
    /// The ID of the AFK channel.
    pub afk_channel_id: Option<String>,
    /// The AFK timeout in seconds.
    pub afk_timeout: usize,
    /// Whether the widget is enabled.
    #[nserde(default)]
    pub widget_enabled: bool,
    /// The ID of the widget channel.
    pub widget_channel_id: Option<String>,
    /// The verification level of the guild.
    pub verification_level: usize,
    /// The default message notifications level.
    #[nserde(rename = "default_message_notifications")]
    pub default_message_notifications_level: usize,
    /// The explicit content filter level.
    #[nserde(rename = "explicit_content_filter")]
    pub explicit_content_filter_level: usize,
    /// The MFA level required for the guild.
    pub mfa_level: usize,
    /// The application ID of the guild.
    pub application_id: Option<String>,
    /// The ID of the system channel.
    pub system_channel_id: Option<String>,
    /// The system channel flags.
    pub system_channel_flag: Option<usize>,
    /// The ID of the rules channel.
    pub rules_channel_id: Option<String>,
    /// The maximum number of members in the guild.
    #[nserde(default)]
    pub max_members: usize,
    /// The vanity URL code of the guild.
    pub vanity_url_code: Option<String>,
    /// The hash of the guild banner.
    #[nserde(rename = "banner")]
    pub banner_hash: Option<String>,
    /// The number of boosts the guild has.
    #[nserde(default, rename = "premium_subscription_count")]
    pub boost_count: Option<u32>,
    /// The preferred locale of the guild.
    pub preferred_locale: String,
    /// The ID of the public updates channel.
    pub public_updates_channel_id: Option<String>,
    /// The maximum number of users in a video channel.
    pub max_video_channel_users: Option<usize>,
    /// The NSFW level of the guild.
    pub nsfw_level: usize,
    /// Whether the boost progress bar is enabled.
    #[nserde(rename = "premium_progress_bar_enabled")]
    pub boost_progress_bar: bool,
    /// The ID of the safety alerts channel.
    pub safety_alerts_channel_id: Option<String>,
    /// The timestamp when the guild was joined.
    pub joined_at: String,
    /// Whether the guild is large.
    pub large: bool,
    /// Whether the guild is unavailable.
    pub unavailable: Option<bool>,
    /// The number of members in the guild.
    pub member_count: usize,
    /// The members of the guild.
    pub members: Vec<Member>,
    /// The channels in the guild.
    pub channels: Vec<Channel>,
    /// The threads in the guild.
    pub threads: Vec<Channel>,
}

/// Represents a partial guild (unavailable guild).
#[doc(alias = "UnavailableGuild")]
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct PartialGuild {
    /// Whether the guild is unavailable.
    pub unavailable: bool,
    /// The unique ID of the guild.
    pub id: String,
}

/// Represents a member who left the guild (kick/leave/ban)
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct MemberLeave {
    // TODO: better naming?
    /// ID of the guild
    pub guild_id: Option<String>,
    /// The user who was removed
    pub user: User,
}

/// Represents a member of a guild.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Member {
    /// The user associated with the member.
    pub user: Option<User>,
    /// The nickname of the member.
    #[nserde(default)]
    pub nick: Option<String>,
    /// The hash of the member's guild avatar.
    #[nserde(rename = "avatar")]
    pub guild_avatar_hash: Option<String>,
    /// The roles of the member.
    #[nserde(default)]
    pub roles: Vec<String>,
    /// The timestamp when the member joined the guild.
    /// `None` if its from guild member add event.
    #[nserde(default)]
    pub joined_at: Option<String>,
    /// The timestamp when the member started boosting the guild.
    #[nserde(default)]
    pub premium_since: Option<String>,
    /// Whether the member is deafened.
    #[nserde(default)]
    pub deaf: bool,
    /// Whether the member is muted.
    #[nserde(default)]
    pub mute: bool,
    /// The flags of the member.
    #[nserde(default)]
    pub flags: usize,
    /// Whether the member is pending verification.
    #[nserde(default)]
    pub pending: Option<bool>,
    /// The permissions of the member.
    #[nserde(default)]
    pub permissions: Option<String>,
    /// The timestamp when the member's communication is disabled until.
    #[nserde(rename = "communication_disabled_until")]
    pub timeout_until: Option<String>,
    // #[nserde(rename = "avatar_decoration_data")]
    // pub avatar_decoration: Option<AvatarDecorationData>,
    /// The mention string for the member.
    #[nserde(default)]
    pub mention: String,

    /// Id of the guild, available in guild member update events.
    #[nserde(default)]
    pub guild_id: Option<String>,
}

impl Guild {
    /// Fetches a member of the guild by user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    ///
    /// # Examples
    ///
    /// ```
    /// let member = guild.fetch_member("user_id").await?;
    /// ```
    pub async fn fetch_member(&self, user_id: &str) -> Result<Member, Box<dyn std::error::Error>> {
        utils::fetch_member(&self.id, user_id).await
    }

    /// Fetches a role of the guild by role ID.
    ///
    /// # Arguments
    ///
    /// * `role_id` - The ID of the role.
    ///
    /// # Examples
    ///
    /// ```
    /// let role = guild.fetch_role("role_id").await?;
    /// ```
    pub async fn fetch_role(&self, role_id: &str) -> Result<Role, Box<dyn std::error::Error>> {
        utils::fetch_role(&self.id, role_id).await
    }

    /// Fetches the default role of the guild.
    ///
    /// # Examples
    ///
    /// ```
    /// let default_role = guild.default_role().await?;
    /// ```
    pub async fn default_role(&self) -> Result<Role, Box<dyn std::error::Error>> {
        utils::fetch_role(&self.id, &self.id).await
    }
}

impl Member {
    /// Gets the avatar URL of the member.
    ///
    /// # Arguments
    ///
    /// * `image_format` - The format of the image.
    /// * `size` - The size of the image.
    ///
    /// # Examples
    ///
    /// ```
    /// let avatar_url = member.get_avatar_url(ImageFormat::PNG, Some(128));
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

        let user = self.user.as_ref().unwrap();
        let user_id = &user.id;
        let avatar_hash = user.avatar_hash.as_ref()?;

        Some(format!(
            "{DISCORD_CDN}/avatars/{user_id}/{avatar_hash}{image_format}{size}"
        ))
    }

    /// Sends a direct message to the member.
    ///
    /// # Arguments
    ///
    /// * `data` - The data for the message.
    ///
    /// # Examples
    ///
    /// ```
    /// member.send_dm("Hello, world!").await;
    /// ```
    pub async fn send_dm(&self, data: impl Into<CreateMessageData>) {
        utils::send_dm(&self.user.as_ref().unwrap().id, data).await;
    }
}
