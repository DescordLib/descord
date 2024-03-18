use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Default, Debug, Clone)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub video: Option<EmbedVideo>,

    #[nserde(default)]
    pub fields: Vec<EmbedField>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(DeJson, SerJson, Debug, Clone, Default)]
pub struct EmbedField {
    pub name: String,
    pub value: String,

    #[nserde(default)]
    pub inline: bool,
}
