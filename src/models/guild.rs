use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson)]
pub struct Guild {
    pub unavailable: bool,
    pub id: String,
}
