use crate::consts::ChannelType;

use super::*;

/// Get a channel by ID
pub async fn fetch_channel(channel_id: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    // check if channel is in cache
    // if let Some(channel) = CHANNEL_CACHE.lock().unwrap().get(channel_id).cloned() {
        // return Ok(channel);
    // }

    let url = format!("channels/{channel_id}");
    let resp = request(Method::GET, &url, None).await.text().await?;
    let mut channel = Channel::deserialize_json(&resp).map_err(|e| e.into());

    if let Ok(i) = channel.as_mut() {
        i.id = channel_id.to_owned();
    }

    if let Ok(channel) = &mut channel {
        channel.mention = format!("<#{}>", channel.id);
    }

    let mut channel_clone = channel.as_ref().cloned().unwrap(); // works
    let channel_id = channel_clone.id.clone();

    // CHANNEL_CACHE.lock().unwrap().put(channel_id, channel_clone);

    channel
}

/// Deletes a channel by ID
/// Deleting a guild channel cannot be undone.
pub async fn delete_channel(channel_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("channels/{channel_id}");
    request(Method::DELETE, &url, None).await.text().await?;

    // CHANNEL_CACHE.lock().unwrap().pop_entry(channel_id);

    Ok(())
}

/// Create a channel in a guild
pub async fn create_channel() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "guilds/{guild.id}/channels";
    todo!();
}

/// Post a typing indicator for the specified channel, which expires after 10 seconds.
pub async fn send_typing(channel_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("channels/{channel_id}/typing");
    request(Method::POST, &url, None).await.text().await?;
    Ok(())
}

type DateTime = chrono::DateTime<chrono::Utc>;

/// Retrieves the messages in a channel.
/// Default limit is 50.
pub async fn get_channel_messages(
    channel_id: &str,
    before: Option<DateTime>,
    around: Option<DateTime>,
    after: Option<DateTime>,
    limit: Option<usize>,
) -> Vec<Message> {
    let limit = limit.unwrap_or(50);
    assert!((1..=100).contains(&limit));

    let mut payload = object! {};

    if let Some(before) = before {}

    todo!();
}
