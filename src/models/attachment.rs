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
    pub waveform: Option<String>,
    pub flags: Option<usize>,
}

#[derive(Debug, DeJson, SerJson, Clone)]
pub struct AttachmentPayload {
    pub file_name: String,
    pub file_path: String,
    pub mime_type: String,
}

impl AttachmentPayload {
    pub fn new(file_name: &str, file_path: &str, mime_type: &str) -> Self {
        AttachmentPayload {
            mime_type: mime_type.to_owned(),
            file_name: file_name.to_owned(),
            file_path: file_path.to_owned(),
        }
    }
}
