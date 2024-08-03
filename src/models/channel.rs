use crate::utils;
use nanoserde::{DeJson, SerJson};

use super::message_response::{CreateMessageData, Message};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Channel {
    pub id: String,
    #[nserde(rename = "type")]
    pub channel_type: u32,
    pub guild_id: Option<String>,
    pub position: Option<usize>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub rate_limit_per_user: Option<usize>,
    // pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u32>,
    pub message_count: Option<u32>,
    pub member_count: Option<u32>,
    // pub thread_metadata: Option<ThreadMetadata>,
    // pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u32>,
    pub permissions: Option<String>,
    pub flags: Option<usize>,
    pub total_message_sent: Option<u32>,
    // pub available_tags: Option<Vec<Tag>>,
    pub applied_tags: Option<Vec<String>>,
    // pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<u32>,
    pub default_sort_order: Option<u32>,
    pub default_forum_layout: Option<u32>,
    #[nserde(default)]
    pub mention: String,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Overwrite {
    pub id: String,
    #[nserde(rename = "type")]
    pub overwrite_type: u32,
    pub allow: String,
    pub deny: String,
}

impl Channel {
    pub async fn send_message(&self, data: impl Into<CreateMessageData>) -> Message {
        utils::send(&self.id, None, data).await
    }

    pub async fn send_typing(&self) {
        utils::send_typing(&self.id).await;
    }
}
