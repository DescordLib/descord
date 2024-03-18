use nanoserde::{SerJson, DeJson};

#[derive(DeJson, SerJson)]
pub struct DeletedMessageResponse {
    #[nserde(rename = "d")]
    pub data: DeletedMessageData
}

#[derive(DeJson, SerJson)]
pub struct DeletedMessageData {
    #[nserde(rename = "id")]
    pub message_id: String,
    pub channel_id: String,
    pub guild_id: String,
}
