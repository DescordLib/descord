use crate::models::role::Role;
use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleCreateResponse {
    #[nserde(rename = "d")]
    pub data: RoleEvent,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleUpdateResponse {
    #[nserde(rename = "d")]
    pub data: RoleEvent,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleDeleteResponse {
    #[nserde(rename = "d")]
    pub data: RoleDelete,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleEvent {
    pub guild_id: String,
    pub role: Role,
}

#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleDelete {
    pub guild_id: String,
    pub role_id: String,
}
