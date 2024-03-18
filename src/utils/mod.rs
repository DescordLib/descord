use crate::consts::API;
use crate::models::channel::Channel;
use crate::models::message_response::CreateMessageData;
use crate::prelude::MessageData;
use crate::{client::TOKEN, models::message_edit::MessageEditData};

use futures_util::TryFutureExt;
use json::object;
use nanoserde::{DeJson, SerJson};
use reqwest::Client;
use reqwest::{header::HeaderMap, Response, StatusCode};

// TODO: Error checking in each function

pub async fn send(channel_id: &str, data: impl Into<CreateMessageData>) {
    let data = data.into();
    let body = data.serialize_json();

    let url = format!("{API}/channels/{channel_id}/messages");

    Client::new()
        .post(url)
        .body(json::stringify(body))
        .headers(get_headers())
        .send()
        .await
        .expect("Failed to send http request");
}

pub async fn reply(reply_to: &MessageData, data: impl Into<CreateMessageData>) -> MessageData {
    let data: CreateMessageData = data.into();

    let mut body = json::parse(&data.serialize_json()).unwrap();
    body.insert(
        "message_reference",
        object! {
            message_id: reply_to.message_id.as_str(),
            guild_id: reply_to.guild_id.as_ref().unwrap().as_str(),
        },
    );

    let url = format!(
        "{API}/channels/{channel_id}/messages",
        channel_id = reply_to.channel_id
    );

    let resp = reqwest::Client::new()
        .post(url)
        .body(json::stringify(body))
        .headers(get_headers())
        .send()
        .await
        .expect("Failed to send http request")
        .text()
        .await
        .unwrap();

    MessageData::deserialize_json(&resp).unwrap()
}

pub async fn get_channel(channel_id: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let url = format!("{API}/channels/{channel_id}");

    let resp = reqwest::Client::new()
        .get(url)
        .headers(get_headers())
        .send()
        .await?;

    Ok(Channel::deserialize_json(&resp.text().await?)?)
}

/// Returns true if the operation was successful, false otherwise.
/// This function requires the MANAGE_MESSAGES permission.
pub async fn delete_message(channel_id: &str, message_id: &str) -> bool {
    let url = format!("{API}/channels/{channel_id}/messages/{message_id}");

    let resp = reqwest::Client::new()
        .delete(url)
        .headers(get_headers())
        .send()
        .await
        .unwrap();

    resp.status() == StatusCode::NO_CONTENT
}

pub async fn edit_message(channel_id: &str, message_id: &str, data: impl Into<MessageEditData>) {
    let url = format!("{API}/channels/{channel_id}/messages/{message_id}");
    let data: MessageEditData = data.into();

    Client::new()
        .patch(url)
        .body(data.serialize_json())
        .headers(get_headers())
        .send()
        .await
        .unwrap();
}

fn get_headers() -> HeaderMap {
    let mut map = HeaderMap::new();

    map.insert("Content-Type", "application/json".parse().unwrap());
    map.insert(
        "Authorization",
        format!("Bot {}", TOKEN.lock().unwrap().as_ref().unwrap())
            .parse()
            .unwrap(),
    );

    map
}
