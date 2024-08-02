use super::*;

/// Get a channel by ID
pub async fn fetch_channel(channel_id: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    // check if channel is in cache
    if let Some(channel) = CHANNEL_CACHE.lock().await.get(channel_id).cloned() {
        return Ok(channel);
    }

    let url = format!("channels/{channel_id}");
    let resp = request(Method::GET, &url, None).await.text().await?;
    let mut channel = Channel::deserialize_json(&resp).map_err(|e| e.into());

    if let Ok(channel) = &mut channel {
        channel.mention = format!("<#{}>", channel.id);
    }

    // why its not working?

    // put channel in cache
    // if let Ok(channel) = channel.as_ref() {
    //     let clone = channel.clone();
    //     let id = channel_id.to_string();
    //     CHANNEL_CACHE.lock().await.put(id, clone);
    // }

    channel
}

/// Deletes a channel by ID
/// Deleting a guild channel cannot be undone.
pub async fn delete_channel(channel_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("channels/{channel_id}");
    request(Method::DELETE, &url, None).await.text().await?;

    // remove channel from cache
    CHANNEL_CACHE.lock().await.pop_entry(channel_id);

    Ok(())
}

/// Post a typing indicator for the specified channel, which expires after 10 seconds.
pub async fn send_typing(channel_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("channels/{channel_id}/typing");
    request(Method::POST, &url, None).await.text().await?;
    Ok(())
}
