use nanoserde::{DeJson, SerJson};

#[derive(Debug, DeJson, SerJson, Clone)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: usize,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f32>,
    pub waveform: String,
    pub flags: Option<usize>,
}
