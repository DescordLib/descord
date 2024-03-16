use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson)]
pub struct EmbedData {
    #[nserde(rename = "type")]
    pub embed_type: String,
    pub timestamp: String,
    pub footer: FooterData,
    pub description: String,
    pub color: usize,
    pub author: EmbedAuthorData,
}

#[derive(DeJson, SerJson)]
pub struct EmbedAuthorData {
    pub proxy_icon_url: String,
    pub name: String,
    pub icon_url: String,
}

#[derive(DeJson, SerJson)]
pub struct FooterData {
    pub text: String,
}
