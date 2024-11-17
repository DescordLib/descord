use crate::utils;
use nanoserde::{DeJson, SerJson};

use super::message_response::{CreateMessageData, Message};

/// Represents a Discord channel.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct Channel {
    /// The unique ID of the channel.
    #[nserde(default)]
    pub id: String,
    /// The type of the channel.
    #[nserde(default, rename = "type")]
    pub channel_type: u32,
    /// The ID of the guild the channel belongs to.
    pub guild_id: Option<String>,
    /// The position of the channel in the guild.
    pub position: Option<usize>,
    /// The permission overwrites for the channel.
    pub permission_overwrites: Option<Vec<Overwrite>>,
    /// The name of the channel.
    pub name: Option<String>,
    /// The topic of the channel.
    pub topic: Option<String>,
    /// Whether the channel is marked as NSFW.
    pub nsfw: Option<bool>,
    /// The ID of the last message sent in the channel.
    pub last_message_id: Option<String>,
    /// The bitrate of the channel (if it's a voice channel).
    pub bitrate: Option<u32>,
    /// The user limit of the channel (if it's a voice channel).
    pub user_limit: Option<u32>,
    /// The rate limit per user in the channel.
    pub rate_limit_per_user: Option<usize>,
    // pub recipients: Option<Vec<User>>,
    /// The icon of the channel.
    pub icon: Option<String>,
    /// The ID of the owner of the channel.
    pub owner_id: Option<String>,
    /// The application ID of the channel.
    pub application_id: Option<String>,
    /// Whether the channel is managed.
    pub managed: Option<bool>,
    /// The ID of the parent channel.
    pub parent_id: Option<String>,
    /// The timestamp of the last pinned message in the channel.
    pub last_pin_timestamp: Option<String>,
    /// The RTC region of the channel (if it's a voice channel).
    pub rtc_region: Option<String>,
    /// The video quality mode of the channel (if it's a voice channel).
    pub video_quality_mode: Option<u32>,
    /// The message count in the channel.
    pub message_count: Option<u32>,
    /// The member count in the channel.
    pub member_count: Option<u32>,
    // pub thread_metadata: Option<ThreadMetadata>,
    // pub member: Option<ThreadMember>,
    /// The default auto-archive duration for threads in the channel.
    pub default_auto_archive_duration: Option<u32>,
    /// The permissions of the channel.
    pub permissions: Option<String>,
    /// The flags of the channel.
    pub flags: Option<usize>,
    /// The total number of messages sent in the channel.
    pub total_message_sent: Option<u32>,
    // pub available_tags: Option<Vec<Tag>>,
    /// The applied tags in the channel.
    pub applied_tags: Option<Vec<String>>,
    // pub default_reaction_emoji: Option<DefaultReaction>,
    /// The default rate limit per user for threads in the channel.
    pub default_thread_rate_limit_per_user: Option<u32>,
    /// The default sort order for threads in the channel.
    pub default_sort_order: Option<u32>,
    /// The default forum layout for the channel.
    pub default_forum_layout: Option<u32>,
    /// The mention string for the channel.
    #[nserde(default)]
    pub mention: String,
}

/// Represents a permission overwrite for a channel.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Overwrite {
    /// The unique ID of the overwrite.
    pub id: String,
    /// The type of the overwrite.
    #[nserde(rename = "type")]
    pub overwrite_type: u32,
    /// The allowed permissions.
    pub allow: String,
    /// The denied permissions.
    pub deny: String,
}

impl Channel {
    /// Sends a message to the channel.
    ///
    /// # Arguments
    ///
    /// * `data` - The data for the message.
    ///
    /// # Examples
    ///
    /// ```
    /// channel.send_message("Hello, world!").await;
    /// ```
    pub async fn send_message(&self, data: impl Into<CreateMessageData>) -> Message {
        utils::send(&self.id, None, data).await
    }

    /// Sends a typing indicator to the channel.
    ///
    /// # Examples
    ///
    /// ```
    /// channel.send_typing().await;
    /// ```
    pub async fn send_typing(&self) {
        utils::send_typing(&self.id).await;
    }
}
