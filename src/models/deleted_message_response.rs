use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug)]
pub struct DeletedMessageResponse {
    #[nserde(rename = "d")]
    pub data: DeletedMessage,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct DeletedMessage {
    #[nserde(rename = "id")]
    pub message_id: String,
    pub channel_id: String,
    pub guild_id: String,
}
