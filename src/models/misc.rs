use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct ResponseWrapper<T: DeJson + SerJson> {
    #[nserde(rename = "d")]
    pub data: T,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct Reconnect {
    /// Always None
    pub data: Option<()>,
}
