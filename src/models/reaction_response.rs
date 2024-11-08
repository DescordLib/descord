use nanoserde::{DeJson, SerJson};

use crate::utils;

use super::{channel::Channel, emoji::Emoji, guild::Member, message_response::Message, user::User};

/// Represents a response for a reaction.
#[derive(DeJson, SerJson, Clone, Debug)]
pub struct ReactionResponse {
    /// The reaction data.
    #[nserde(rename = "d")]
    pub data: Reaction,
}

/// Represents a reaction in Discord.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Reaction {
    /// The ID of the user who reacted.
    pub user_id: String,
    /// The ID of the message that was reacted to.
    pub message_id: String,
    /// The member who reacted.
    #[nserde(default)]
    pub member: Option<Member>,
    /// The emoji used for the reaction.
    pub emoji: Emoji,
    /// The ID of the channel where the reaction occurred.
    pub channel_id: String,
    /// Whether the reaction is a burst reaction.
    pub burst: bool,
    /// The ID of the guild where the reaction occurred.
    #[nserde(default)]
    pub guild_id: Option<String>,
}

impl Reaction {
    /// Fetches the channel where the reaction occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// let channel = reaction.get_channel().await?;
    /// ```
    pub async fn get_channel(&self) -> Result<Channel, Box<dyn std::error::Error>> {
        utils::fetch_channel(&self.channel_id).await
    }

    /// Fetches the user who reacted.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = reaction.get_user().await?;
    /// ```
    pub async fn get_user(&self) -> Result<User, Box<dyn std::error::Error>> {
        utils::fetch_user(&self.user_id).await
    }

    /// Fetches the message that was reacted to.
    ///
    /// # Examples
    ///
    /// ```
    /// let message = reaction.get_message().await?;
    /// ```
    pub async fn get_message(&self) -> Result<Message, Box<dyn std::error::Error>> {
        utils::fetch_message(&self.channel_id, &self.message_id).await
    }

    /// Removes the reaction.
    ///
    /// # Examples
    ///
    /// ```
    /// reaction.remove_reaction().await;
    /// ```
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
