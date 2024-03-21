use nanoserde::{DeJson, SerJson};

use super::{user::User, channel::Channel};

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
    pub system_channel_flag: usize,
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
    // TODO: permissions, roles, welcome_screen, sticker
}

#[derive(DeJson, SerJson)]
pub struct GuildCreateResponse {
    #[nserde(rename = "d")]
    pub data: GuildCreateData,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct GuildCreateData {
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
    pub members: Vec<GuildMember>,
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
pub struct GuildMember {
    pub roles: Vec<String>,
    pub mute: bool,
    pub joined_at: String,

    pub pending: Option<bool>,
    pub user: Option<User>,
    #[nserde(default, rename = "avatar")]
    pub guild_avatar_hash: Option<String>,
    #[nserde(default)]
    pub flags: usize,
    pub premium_since: Option<String>,
    pub nick: Option<String>,
}
