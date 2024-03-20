use nanoserde::{DeJson, SerJson};

use crate::utils;

use super::{
    channel::Channel, emoji::Emoji, guild::GuildMember, message_response::Message, user::User,
};

#[derive(DeJson, SerJson, Clone, Debug)]
pub struct ReactionResponse {
    #[nserde(rename = "d")]
    pub data: Reaction,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Reaction {
    pub user_id: String,
    pub message_id: String,
    #[nserde(default)]
    pub member: Option<GuildMember>,
    pub emoji: Emoji,
    pub channel_id: String,
    pub burst: bool,
    #[nserde(default)]
    pub guild_id: Option<String>,
}

impl Reaction {
    pub async fn get_channel(&self) -> Result<Channel, Box<dyn std::error::Error>> {
        utils::get_channel(&self.channel_id).await
    }

    pub async fn get_user(&self) -> Result<User, Box<dyn std::error::Error>> {
        utils::get_user(&self.user_id).await
    }

    pub async fn get_message(&self) -> Message {
        utils::get_message(&self.channel_id, &self.message_id).await
    }

    pub async fn remove_reaction(&self) {
        utils::remove_reaction(
            &self.channel_id,
            &self.message_id,
            &self.user_id,
            &if let Some(ref id) = self.emoji.id {
                format!("{name}:{id}", name = self.emoji.name)
            } else {
                self.emoji.name.clone()
            },
        )
        .await;
    }
}
