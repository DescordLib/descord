use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug)]
pub struct DeletedMessageResponse {
    #[nserde(rename = "d")]
    pub data: DeletedMessageData,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct DeletedMessageData {
    #[nserde(rename = "id")]
    pub message_id: String,
    pub channel_id: String,
    pub guild_id: String,
}
