use nanoserde::{DeJson, SerJson};

/// Represents an embed object in a Discord message.
#[derive(DeJson, SerJson, Default, Debug, Clone)]
pub struct Embed {
    /// The title of the embed.
    pub title: Option<String>,
    /// The description of the embed.
    pub description: Option<String>,
    /// The URL of the embed.
    pub url: Option<String>,
    /// The timestamp of the embed content.
    pub timestamp: Option<String>,
    /// The color code of the embed.
    pub color: Option<u32>,
    /// The footer information of the embed.
    pub footer: Option<EmbedFooter>,
    /// The image information of the embed.
    pub image: Option<EmbedImage>,
    /// The thumbnail information of the embed.
    pub thumbnail: Option<EmbedThumbnail>,
    /// The provider information of the embed.
    pub provider: Option<EmbedProvider>,
    /// The author information of the embed.
    pub author: Option<EmbedAuthor>,
    /// The video information of the embed.
    pub video: Option<EmbedVideo>,
    /// The fields of the embed.
    #[nserde(default)]
    pub fields: Vec<EmbedField>,
}

/// Represents the footer information of an embed.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedFooter {
    /// The text of the footer.
    pub text: String,
    /// The URL of the footer icon.
    pub icon_url: Option<String>,
    /// The proxy URL of the footer icon.
    pub proxy_icon_url: Option<String>,
}

/// Represents the video information of an embed.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedVideo {
    /// The URL of the video.
    pub url: Option<String>,
    /// The proxy URL of the video.
    pub proxy_url: Option<String>,
    /// The height of the video.
    pub height: Option<u32>,
    /// The width of the video.
    pub width: Option<u32>,
}

/// Represents the image information of an embed.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedImage {
    /// The URL of the image.
    pub url: String,
    /// The proxy URL of the image.
    pub proxy_url: Option<String>,
    /// The height of the image.
    pub height: Option<u32>,
    /// The width of the image.
    pub width: Option<u32>,
}

/// Represents the thumbnail information of an embed.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedThumbnail {
    /// The URL of the thumbnail.
    pub url: String,
    /// The proxy URL of the thumbnail.
    pub proxy_url: Option<String>,
    /// The height of the thumbnail.
    pub height: Option<u32>,
    /// The width of the thumbnail.
    pub width: Option<u32>,
}

/// Represents the provider information of an embed.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedProvider {
    /// The name of the provider.
    pub name: Option<String>,
    /// The URL of the provider.
    pub url: Option<String>,
}

/// Represents the author information of an embed.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedAuthor {
    /// The name of the author.
    pub name: String,
    /// The URL of the author.
    pub url: Option<String>,
    /// The URL of the author icon.
    pub icon_url: Option<String>,
    /// The proxy URL of the author icon.
    pub proxy_icon_url: Option<String>,
}

/// Represents a field in an embed.
#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedField {
    /// The name of the field.
    pub name: String,
    /// The value of the field.
    pub value: String,
    /// Whether the field is inline.
    #[nserde(default)]
    pub inline: bool,
}
