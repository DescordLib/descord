use crate::consts::permissions::*;
use crate::prelude::{Channel, Guild, Member};
use chrono::{DateTime, Utc};

enum MemberOrId {
    Id(u32),
    MemberObj(Member),
}

pub async fn fetch_permissions(member: &Member, guild: &Guild, channel: Option<&Channel>) -> u64 {
    let id = member.clone().user.unwrap_or_default().id;
    if id.is_empty() {
        return 0;
    }

    // Check if member is the guild owner
    if guild.owner_id == id {
        return ADMINISTRATOR;
    }

    // Start with default role permissions or bot-specific permissions
    let mut base_permissions = guild
        .default_role()
        .await
        .unwrap()
        .permissions
        .parse::<u64>()
        .unwrap();

    // Aggregate permissions from member's roles
    for role_id in &member.roles {
        if let Some(role) = guild.fetch_role(role_id).await.ok() {
            base_permissions |= role.permissions.parse::<u64>().unwrap();
        }
    }

    // Administrator check
    if base_permissions & ADMINISTRATOR == ADMINISTRATOR {
        return ADMINISTRATOR;
    }

    // Apply permission overwrites if channel is provided
    if let Some(channel) = channel {
        if let Some(overwrites) = &channel.permission_overwrites {
            for overwrite in overwrites {
                let allow = overwrite.allow.parse::<u64>().unwrap();
                let deny = overwrite.deny.parse::<u64>().unwrap();

                if overwrite.overwrite_type == 1 && overwrite.id == id {
                    // Member specific overwrites
                    base_permissions &= !deny;
                    base_permissions |= allow;
                } else if overwrite.overwrite_type == 0 && member.roles.contains(&overwrite.id) {
                    // Role specific overwrites
                    base_permissions &= !deny;
                    base_permissions |= allow;
                }
            }
        }
    }

    // Timeout check
    if let Some(timeout_str) = &member.timeout_until {
        if let Ok(timeout_time) = DateTime::parse_from_rfc3339(timeout_str) {
            if timeout_time > Utc::now() {
                // Apply mask to retain only VIEW_CHANNEL and READ_MESSAGE_HISTORY permissions
                base_permissions &= VIEW_CHANNEL | READ_MESSAGE_HISTORY;
            }
        }
    }

    base_permissions
}
