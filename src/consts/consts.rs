pub const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";
pub const API: &str = "https://discord.com/api/v10";
pub const MESSAGE_CACHE_SIZE: usize = 100_000;

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
