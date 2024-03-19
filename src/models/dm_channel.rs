use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson)]
pub struct DirectMessageChannel {
    pub id: String,
    #[nserde(rename = "type")]
    pub type_: usize,
    pub last_message_id: Option<String>,
    #[nserde(rename = "icon")]
    pub icon_hash: Option<String>,
    pub application_id: Option<String>,
    #[nserde(default)]
    pub flags: usize,
}
