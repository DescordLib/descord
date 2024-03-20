use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Emoji {
    pub name: String,
    #[nserde(default)]
    pub id: Option<String>,
}
