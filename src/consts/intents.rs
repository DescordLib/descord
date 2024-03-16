#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum GatewayIntent {
    Guilds = 1 << 0,
    GuildMembers = 1 << 1,
    GuildModeration = 1 << 2,
    GuildEmojisAndStickers = 1 << 3,
    GuildIntegrations = 1 << 4,
    GuildWebhooks = 1 << 5,
    GuildInvites = 1 << 6,
    GuildVoiceStates = 1 << 7,
    GuildPresences = 1 << 8,
    GuildMessages = 1 << 9,
    GuildMessageReactions = 1 << 10,
    GuildMessageTyping = 1 << 11,
    DirectMessages = 1 << 12,
    DirectmessageReactions = 1 << 13,
    DirectMessageTyping = 1 << 14,
    MessageContent = 1 << 15,
    GuildScheduledEvents = 1 << 16,
    AutoModerationConfiguration = 1 << 20,
    AutoModerationExecution = 1 << 21,
}

impl Into<u32> for GatewayIntent {
    fn into(self) -> u32 {
        self as _
    }
}

impl std::ops::BitOr for GatewayIntent {
    type Output = u32;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}
