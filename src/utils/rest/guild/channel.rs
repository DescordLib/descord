use super::*;

/// Get a channel by ID
pub async fn fetch_channel(channel_id: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let url = format!("channels/{channel_id}");
    let resp = request(Method::GET, &url, None).await.text().await?;
    let mut channel = Channel::deserialize_json(&resp).map_err(|e| e.into());
    if let Ok(channel) = &mut channel {
        channel.mention = format!("<#{}>", channel.id);
    }
    channel
}
