use crate::models::role::Role;
use nanoserde::{DeJson, SerJson};

/// Represents a response for a role creation event.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleCreateResponse {
    /// The role event data.
    #[nserde(rename = "d")]
    pub data: RoleEvent,
}

/// Represents a response for a role update event.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleUpdateResponse {
    /// The role event data.
    #[nserde(rename = "d")]
    pub data: RoleEvent,
}

/// Represents a response for a role deletion event.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleDeleteResponse {
    /// The role deletion data.
    #[nserde(rename = "d")]
    pub data: RoleDelete,
}

/// Represents a role event in a guild.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleEvent {
    /// The ID of the guild where the event occurred.
    pub guild_id: String,
    /// The role involved in the event.
    pub role: Role,
}

/// Represents a role deletion event in a guild.
#[derive(DeJson, SerJson, Debug, Clone)]
pub struct RoleDelete {
    /// The ID of the guild where the role was deleted.
    pub guild_id: String,
    /// The ID of the deleted role.
    pub role_id: String,
}
