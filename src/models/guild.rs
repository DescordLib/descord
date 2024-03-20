use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug)]
pub struct Guild {
    pub unavailable: bool,
    pub id: String,
}
