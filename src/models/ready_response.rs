use nanoserde::{DeJson, SerJson};

use super::{guild::PartialGuild, user::User};

/// Represents a ready response.
#[derive(DeJson, SerJson, Debug)]
pub struct ReadyResponse {
    #[nserde(rename = "d")]
    pub data: ReadyData,
}

/// Represents general startup data about the bot.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct ReadyData {
    pub user: User,
    pub session_type: String,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub guilds: Vec<PartialGuild>,
    pub geo_ordered_rtc_regions: Vec<String>,
    pub application: ApplicationData,
}

/// Represents data about the bot application.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct ApplicationData {
    pub id: String,
    pub flags: usize,
}
