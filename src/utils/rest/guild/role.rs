use super::*;

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
