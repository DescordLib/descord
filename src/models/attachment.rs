use nanoserde::{DeJson, SerJson};

/// Represents an attachment in a message.
#[derive(Debug, DeJson, SerJson, Clone)]
pub struct Attachment {
    /// The unique ID of the attachment.
    pub id: String,
    /// The filename of the attachment.
    pub filename: String,
    /// The title of the attachment.
    pub title: Option<String>,
    /// The description of the attachment.
    pub description: Option<String>,
    /// The content type of the attachment.
    pub content_type: Option<String>,
    /// The size of the attachment in bytes.
    pub size: usize,
    /// The URL of the attachment.
    pub url: String,
    /// The proxy URL of the attachment.
    pub proxy_url: String,
    /// The height of the attachment (if it's an image).
    pub height: Option<u32>,
    /// The width of the attachment (if it's an image).
    pub width: Option<u32>,
    /// Whether the attachment is ephemeral.
    pub ephemeral: Option<bool>,
    /// The duration of the attachment (if it's a video or audio).
    pub duration_secs: Option<f32>,
    /// The waveform of the attachment (if it's an audio).
    pub waveform: Option<String>,
    /// The flags of the attachment.
    pub flags: Option<usize>,
}

/// Represents the payload for an attachment.
#[derive(Debug, DeJson, SerJson, Clone)]
pub struct AttachmentPayload {
    /// The filename of the attachment.
    pub file_name: String,
    /// The file path of the attachment.
    pub file_path: String,
    /// The MIME type of the attachment.
    pub mime_type: String,
}

impl AttachmentPayload {
    /// Creates a new `AttachmentPayload`.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The filename of the attachment.
    /// * `file_path` - The file path of the attachment.
    /// * `mime_type` - The MIME type of the attachment.
    ///
    /// # Examples
    ///
    /// ```
    /// let payload = AttachmentPayload::new("file.txt", "/path/to/file.txt", "text/plain");
    /// ```
    pub fn new(file_name: &str, file_path: &str, mime_type: &str) -> Self {
        AttachmentPayload {
            mime_type: mime_type.to_owned(),
            file_name: file_name.to_owned(),
            file_path: file_path.to_owned(),
        }
    }
}
