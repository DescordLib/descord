#[allow(non_snake_case)]
pub mod GatewayIntent {
    pub const GUILDS: u32 = 1 << 0;
    pub const GUILD_MEMBERS: u32 = 1 << 1;
    pub const GUILD_MODERATION: u32 = 1 << 2;
    pub const GUILD_EMOJIS_AND_STICKERS: u32 = 1 << 3;
    pub const GUILD_INTEGRATIONS: u32 = 1 << 4;
    pub const GUILD_WEBHOOKS: u32 = 1 << 5;
    pub const GUILD_INVITES: u32 = 1 << 6;
    pub const GUILD_VOICE_STATES: u32 = 1 << 7;
    pub const GUILD_PRESENCES: u32 = 1 << 8;
    pub const GUILD_MESSAGES: u32 = 1 << 9;
    pub const GUILD_MESSAGE_REACTIONS: u32 = 1 << 10;
    pub const GUILD_MESSAGE_TYPING: u32 = 1 << 11;
    pub const DIRECT_MESSAGES: u32 = 1 << 12;
    pub const DIRECTMESSAGE_REACTIONS: u32 = 1 << 13;
    pub const DIRECT_MESSAGE_TYPING: u32 = 1 << 14;
    pub const MESSAGE_CONTENT: u32 = 1 << 15;
    pub const GUILD_SCHEDULED_EVENTS: u32 = 1 << 16;
    pub const AUTO_MODERATION_CONFIGURATION: u32 = 1 << 20;
    pub const AUTO_MODERATION_EXECUTION: u32 = 1 << 21;
}
