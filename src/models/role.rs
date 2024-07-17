use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: i32,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
    pub flags: u32,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleTags {
    pub bot_id: Option<String>,
    pub integration_id: Option<String>,
    pub premium_subscriber: Option<()>,
    pub subscription_listing_id: Option<String>,
    pub available_for_purchase: Option<()>,
    pub guild_connections_id: Option<()>,
}
