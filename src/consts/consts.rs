pub const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";
pub const API: &str = "https://discord.com/api/v10";
pub const MESSAGE_CACHE_SIZE: usize = 100_000;
pub const DISCORD_CDN: &str = "https://cdn.discordapp.com";

#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    Jpeg,
    Png,
    WebP,
    Gif,
    Lottie,
}

impl ImageFormat {
    pub fn get_extension(&self) -> &'static str {
        match self {
            Self::Jpeg => ".jpg",
            Self::Png => ".png",
            Self::WebP => ".webp",
            Self::Gif => ".gif",
            Self::Lottie => ".json",
        }
    }
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_extension())
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    /// Container for other components
    ActionRow = 1,

    /// Button object
    Button,

    /// Select menu for picking from defined text options
    StringSelect,

    /// Text input object
    TextInput,

    /// Select menu for users
    UserSelect,

    /// Select menu for roles
    RoleSelect,

    /// Select menu for mentionables (users and roles)
    MentionableSelect,

    /// Select menu for channels
    ChannelSelect,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// For more information about the button styles
/// head over to https://discord.com/developers/docs/interactions/message-components#button-object-button-styles
pub enum ButtonStyle {
    /// Color: blurple
    Primary = 1,

    /// Color: grey
    Secondary,

    /// Color: green
    Success,

    /// Color: red
    Danger,

    /// Color: grey, navigates to a URL
    Link,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
    GuildMedia = 16,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectMenuType {
    #[default]
    StringSelect = 3,
    TextInput,
    UserSelect,
    RoleSelect,
    MentionableSelect,
    ChannelSelect,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand,
    MessageComponent,
    ApplicationCommandAutocomplete,
    ModalSubmit,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionContextType {
    Guild,
    BotDm,
    PrivateChanne,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
    PremiumRequired = 10,
}
