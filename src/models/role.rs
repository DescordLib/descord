use nanoserde::{DeJson, SerJson};

/// Represents a role in a Discord guild.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Role {
    /// The unique ID of the role.
    pub id: String,
    /// The name of the role.
    pub name: String,
    /// The color of the role.
    pub color: u32,
    /// Whether the role is hoisted (displayed separately in the member list).
    pub hoist: bool,
    /// The icon of the role.
    pub icon: Option<String>,
    /// The unicode emoji of the role.
    pub unicode_emoji: Option<String>,
    /// The position of the role in the guild.
    pub position: i32,
    /// The permissions of the role.
    pub permissions: String,
    /// Whether the role is managed by an integration.
    pub managed: bool,
    /// Whether the role is mentionable.
    pub mentionable: bool,
    /// The tags associated with the role.
    pub tags: Option<RoleTags>,
    /// The flags of the role.
    pub flags: u32,
}

/// Represents the tags associated with a role.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleTags {
    /// The ID of the bot associated with the role.
    pub bot_id: Option<String>,
    /// The ID of the integration associated with the role.
    pub integration_id: Option<String>,
    /// Whether the role is for premium subscribers.
    pub premium_subscriber: Option<()>,
    /// The ID of the subscription listing associated with the role.
    pub subscription_listing_id: Option<String>,
    /// Whether the role is available for purchase.
    pub available_for_purchase: Option<()>,
    /// The ID of the guild connections associated with the role.
    pub guild_connections_id: Option<()>,
}
