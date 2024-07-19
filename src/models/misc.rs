use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Reconnect {
    /// Always None
    pub data: Option<()>,
}
