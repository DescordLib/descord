use std::{borrow::Cow, io::Read};

use reqwest::multipart::{Form, Part};

use crate::models::attachment::AttachmentPayload;

use super::*;

// TODO: fix caching for messages

/// Edit a message sent by the bot in a channel.
///
/// # Arguments
/// `channel_id` - The ID of the channel the message is in
/// `message_id` - The ID of the message to edit
/// `data` - The data to edit the message with
pub async fn edit_message(channel_id: &str, message_id: &str, data: impl Into<CreateMessageData>) {
    let url = format!("channels/{channel_id}/messages/{message_id}");
    let data: CreateMessageData = data.into();

    request(
        Method::PATCH,
        &url,
        Some(json::parse(&data.to_json()).unwrap()),
    )
    .await;
}

/// Gets a message by ID.
///
/// # Arguments
/// `channel_id` - The ID of the channel the message is in
/// `message_id` - The ID of the message to fetch
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

/// Adds a reaction to a message.
///
/// # Arguments
/// `channel_id` - The ID of the channel the message is in
/// `message_id` - The ID of the message to add the reaction to
/// `user_id` - The ID of the user who's reaction is to be removed
/// `emoji` - The emoji to react with
pub async fn remove_reaction(channel_id: &str, message_id: &str, user_id: &str, emoji: &str) {
    let url = format!("channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}");
    request(Method::DELETE, &url, None).await;
}

/// Adds a reaction to a message.
pub async fn react(channel_id: &str, message_id: &str, emoji: &str) {
    let url = format!(
        "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
        emoji = emoji.trim_matches(['<', '>', ':'])
    );

    request(Method::PUT, &url, None).await;
}

/// Send (or reply to) a message to a channel.
///
/// # Arguments
/// `channel_id` - The ID of the channel to send the message to.
/// `reference_message_id` - The ID of the message to reply to, `None` if not replying.
/// `data` - The data to send the message with.
pub async fn send(
    channel_id: &str,
    reference_message_id: Option<&str>,
    data: impl Into<CreateMessageData>,
) -> Message {
    let data: CreateMessageData = data.into();

    let mut body = json::parse(&data.to_json()).unwrap();

    if let Some(message_id) = reference_message_id {
        body.insert(
            "message_reference",
            object! {
                message_id: message_id,
                channel_id: channel_id,
            },
        );
    }

    let endpoint = format!("channels/{channel_id}/messages");
    let multipart = get_message_multipart(channel_id, data.attachments, Some(body.dump()))
        .await
        .unwrap();

    let mut headers = get_headers();
    headers.remove("Content-Type");

    let client = reqwest::Client::new()
        .post(&format!("{API}/{endpoint}"))
        .headers(headers);

    // Cannot clone it
    // let mut response = client
    //     .try_clone()
    //     .unwrap()
    //     .multipart(multipart)
    //     .send()
    //     .await
    //     .unwrap();

    let response = client.multipart(multipart).send().await.unwrap();
    let response_text = response.text().await.unwrap();

    Message::deserialize_json(&response_text.to_string()).unwrap_or_else(|e| {
        panic!(
            "Failed to send message, error: {}",
            json::parse(&response_text).unwrap()["message"]
                .as_str()
                .unwrap()
        )
    })
}

pub async fn get_message_multipart(
    channel_id: &str,
    attachments: Vec<AttachmentPayload>,
    payload_json: Option<String>,
) -> Result<Form, Box<dyn std::error::Error>> {
    let mut form = Form::new();

    if let Some(payload_json) = payload_json {
        let msg = Part::text(Cow::Owned(payload_json.to_string()))
            .mime_str("application/json")
            .unwrap();

        form = form.part("payload_json", msg);
    }

    for (
        file_idx,
        AttachmentPayload {
            file_name,
            file_path,
            mime_type,
        },
    ) in attachments.iter().enumerate()
    {
        let mut file = std::fs::File::open(file_path)?;
        let mut buf = Vec::new();

        file.read_to_end(&mut buf)?;

        let file_part = Part::bytes(Cow::Owned(buf))
            .file_name(Cow::Owned(file_name.to_string()))
            .mime_str(&mime_type)
            .unwrap();

        form = form.part(Cow::Owned(format!("file[{file_idx}]")), file_part);
    }

    Ok(form)

    // let client = reqwest::Client::new()
    //     .post(&format!("{API}/channels/{}/messages", channel_id))
    //     .multipart(form)
    //     .headers(headers);

    // let response = client.send().await?;

    // if response.status().is_success() {
    //     Ok(())
    // } else {
    //     Err(format!("Failed to send attachment: {}", response.text().await?).into())
    // }
}
