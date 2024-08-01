use super::*;

pub async fn edit_message(channel_id: &str, message_id: &str, data: impl Into<CreateMessageData>) {
    let url = format!("channels/{channel_id}/messages/{message_id}");
    let data: CreateMessageData = data.into();

    request(
        Method::PATCH,
        &url,
        Some(json::parse(&data.serialize_json()).unwrap()),
    )
    .await;
}

pub async fn fetch_message(
    channel_id: &str,
    message_id: &str,
) -> Result<Message, Box<dyn std::error::Error>> {
    if let Some(message) = MESSAGE_CACHE.lock().await.get(message_id).cloned() {
        return Ok(message);
    }

    let url = format!("channels/{channel_id}/messages/{message_id}");
    let resp = request(Method::GET, &url, None).await.text().await.unwrap();

    Message::deserialize_json(&resp).map_err(|e| e.into())
}

/// Returns true if the operation was successful, false otherwise.
/// This function requires the MANAGE_MESSAGES permission.
pub async fn delete_message(channel_id: &str, message_id: &str) -> bool {
    let url = format!("channels/{channel_id}/messages/{message_id}");

    let resp = request(Method::DELETE, &url, None).await;
    resp.status() == StatusCode::NO_CONTENT
}

pub async fn remove_reaction(channel_id: &str, message_id: &str, user_id: &str, emoji: &str) {
    let url = format!("channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}");
    request(Method::DELETE, &url, None).await;
}

pub async fn react(channel_id: &str, message_id: &str, emoji: &str) {
    let url = format!(
        "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
        emoji = emoji.trim_matches(['<', '>', ':'])
    );

    request(Method::PUT, &url, None).await;
}

pub async fn reply(
    message_id: &str,
    channel_id: &str,
    data: impl Into<CreateMessageData>,
) -> Message {
    let data: CreateMessageData = data.into();
    let mut body = json::parse(&data.to_json()).unwrap();
    body.insert(
        "message_reference",
        object! {
            message_id: message_id,
        },
    );

    let url = format!("channels/{channel_id}/messages",);

    let resp = request(Method::POST, &url, Some(body))
        .await
        .text()
        .await
        .unwrap();

    Message::deserialize_json(&resp).unwrap()
}

pub async fn send(channel_id: &str, data: impl Into<CreateMessageData>) -> Message {
    let data: CreateMessageData = data.into();
    let body = data.to_json();

    let url = format!("channels/{channel_id}/messages");

    let resp = request(Method::POST, &url, Some(json::parse(&body).unwrap()))
        .await
        .text()
        .await
        .unwrap();

    Message::deserialize_json(&resp).unwrap()
}
