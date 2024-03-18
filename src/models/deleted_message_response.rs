use nanoserde::{SerJson, DeJson};

#[derive(DeJson, SerJson)]
pub struct DeletedMessageResponse {
    #[nserde(rename = "d")]
    data: DeletedMessageData
}

#[derive(DeJson, SerJson)]
pub struct DeletedMessageData {
    id: String,
    channel_id: String,
    guild_id: String,
}
