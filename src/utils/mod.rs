use crate::client::TOKEN;
use crate::consts;
use crate::models::channel::Channel;
use crate::models::message_response::CreateMessageData;
use crate::prelude::MessageData;

use json::object;
use nanoserde::{DeJson, SerJson};
use reqwest::Response;

pub async fn send(channel_id: &str, data: impl Into<CreateMessageData>) {
    let data = data.into();
    let body = data.serialize_json();

    let post_url = format!("{api}/channels/{channel_id}/messages", api = consts::API);

    send_req(&post_url, body)
        .await
        .expect("Failed to send http request");
}

pub async fn reply(reply_to: &MessageData, data: impl Into<CreateMessageData>) {
    let data: CreateMessageData = data.into();

    let mut body = json::parse(&data.serialize_json()).unwrap();
    body.insert(
        "message_reference",
        object! {
            message_id: reply_to.message_id.as_str(),
            guild_id: reply_to.guild_id.as_str(),
        },
    );

    let post_url = format!(
        "{api}/channels/{channel_id}/messages",
        api = consts::API,
        channel_id = reply_to.channel_id
    );

    send_req(&post_url, json::stringify(body))
        .await
        .expect("Failed to send http request");
}

pub async fn get_channel(channel_id: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let post_url = &format!("{api}/channels/{channel_id}", api = consts::API);

    let resp = reqwest::Client::new()
        .get(post_url)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            &format!("Bot {}", TOKEN.lock().unwrap().as_ref().unwrap()),
        )
        .send()
        .await?;

    Ok(Channel::deserialize_json(&resp.text().await?)?)
}

async fn send_req(url: &str, body: String) -> reqwest::Result<Response> {
    reqwest::Client::new()
        .post(url)
        .body(body)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            &format!("Bot {}", TOKEN.lock().unwrap().as_ref().unwrap()),
        )
        .send()
        .await
}
