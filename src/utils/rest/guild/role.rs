use super::*;

/// Get all roles in a guild
///
/// # Arguments
/// guild_id - The ID of the guild to get roles from
pub async fn fetch_roles(guild_id: &str) -> Result<Vec<Role>, Box<dyn std::error::Error>> {
    let url = format!("guilds/{guild_id}/roles");
    let resp = request(Method::GET, &url, None).await.text().await.unwrap();
    let roles: Vec<Role> = DeJson::deserialize_json(&resp).unwrap();

    for role in &roles {
        ROLE_CACHE.lock().await.put(role.id.clone(), role.clone());
    }

    Ok(roles)
}

/// Fetches a particular role in a guild
///
/// # Arguments
/// guild_id - The ID of the guild the role is in
/// role_id - The ID of the role to get
pub async fn fetch_role(guild_id: &str, role_id: &str) -> Result<Role, Box<dyn std::error::Error>> {
    if let Some(role) = ROLE_CACHE.lock().await.get(role_id).cloned() {
        info!("Role cache hit");
        return Ok(role);
    }
    let url = format!("guilds/{guild_id}/roles");
    let resp = request(Method::GET, &url, None).await.text().await.unwrap();
    let roles: Vec<Role> = DeJson::deserialize_json(&resp).unwrap();
    let mut answer = None;
    for role in &roles {
        ROLE_CACHE.lock().await.put(role.id.clone(), role.clone());
        if role.id == role_id {
            answer = Some(role.clone());
        }
    }
    if let Some(answer) = answer {
        Ok(answer)
    } else {
        Err("Role not found".into())
    }
}

/// Delete a role in a guild
///
/// # Arguments
/// guild_id - The ID of the guild the role is in
/// role_id - The ID of the role to delete
pub async fn delete_role(guild_id: &str, role_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("guilds/{guild_id}/roles/{role_id}");
    request(Method::DELETE, &url, None).await.text().await?;
    ROLE_CACHE.lock().await.pop_entry(role_id);
    Ok(())
}

/// Edit a role in a guild
///
/// # Arguments
/// guild_id - The ID of the guild the role is in
/// role_id - The ID of the role to edit
/// position - The new position of the role
///
/// # Role position
/// Roles are sorted in descending order of position,
/// with the highest role having the lowest position.
/// Zero being the lowest position.
pub async fn edit_role_position(
    guild_id: &str,
    role_id: &str,
    position: i32,
) -> Result<Role, Box<dyn std::error::Error>> {
    let url = format!("guilds/{guild_id}/roles/{role_id}");
    let body = object! { "position": position };
    let resp = request(Method::PATCH, &url, Some(body))
        .await
        .text()
        .await
        .unwrap();
    let role: Role = DeJson::deserialize_json(&resp).unwrap();
    ROLE_CACHE.lock().await.put(role.id.clone(), role.clone());
    Ok(role)
}

/// Create a new role in a guild
///
/// # Arguments
/// `guild_id` - The ID of the guild to create the role in
/// `name` - The name of the role
/// `permissions` - The bitwise value of permissions
/// `color` - The color of the role
/// `hoist` - Whether the role should be displayed separately in the sidebar
/// `mentionable` - Whether the role should be mentionable
pub async fn create_role(
    guild_id: &str,
    name: &str,
    permissions: i32,
    color: crate::color::Color,
    hoist: bool,
    mentionable: bool,
) -> Result<Role, Box<dyn std::error::Error>> {
    let url = format!("guilds/{guild_id}/roles");

    let color: u32 = color.into();
    let body = object! {
        "name": name,
        "permissions": permissions,
        "color": color,
        "hoist": hoist,
        "mentionable": mentionable,
    };
    let resp = request(Method::POST, &url, Some(body))
        .await
        .text()
        .await
        .unwrap();
    let role: Role = DeJson::deserialize_json(&resp).unwrap();
    ROLE_CACHE.lock().await.put(role.id.clone(), role.clone());
    Ok(role)
}

pub async fn add_role(
    guild_id: &str,
    user_id: &str,
    role_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("guilds/{guild_id}/members/{user_id}/roles/{role_id}");
    let resp = request(Method::PUT, &url, None).await.error_for_status()?;
    Ok(())
}

pub async fn remove_role(
    guild_id: &str,
    user_id: &str,
    role_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("guilds/{guild_id}/members/{user_id}/roles/{role_id}");
    let resp = request(Method::DELETE, &url, None)
        .await
        .error_for_status()?;
    Ok(())
}
