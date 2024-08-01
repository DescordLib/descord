mod channel;
mod message;
mod role;
mod user;

use super::*;

pub use channel::*;
pub use message::*;
pub use role::*;
pub use user::*;

/* Misc functions */

pub async fn fetch_application_commands(bot_id: &str) -> Vec<ApplicationCommand> {
    let resp = request(
        Method::GET,
        format!("applications/{}/commands", bot_id).as_str(),
        None,
    )
    .await
    .text()
    .await
    .unwrap();

    DeJson::deserialize_json(&resp).unwrap_or_else(|e| {
        log::error!("Failed to deserialize JSON: {}", e);
        vec![]
    })
}

pub async fn fetch_guild(guild_id: &str) -> Result<Guild, Box<dyn std::error::Error>> {
    if let Some(guild) = GUILD_CACHE.lock().await.get(guild_id).cloned() {
        return Ok(guild);
    }

    let url = format!("guilds/{guild_id}");
    let resp = request(Method::GET, &url, None).await.text().await?;
    if let Ok(guild) = Guild::deserialize_json(&resp) {
        GUILD_CACHE
            .lock()
            .await
            .put(guild_id.to_string(), guild.clone());
        Ok(guild)
    } else {
        Err("Failed to deserialize JSON".into())
    }
}
