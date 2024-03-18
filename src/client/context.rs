use crate::consts;
use crate::models::channel::Channel;
use crate::models::message_response::CreateMessageData;
use crate::prelude::MessageData;

use json::object;
use nanoserde::{DeJson, SerJson};
use reqwest::Response;

pub struct Context {
    token: String,
}

impl Context {
    pub(crate) fn new(token: String) -> Self {
        Self {
            token: token.into(),
        }
    }

    pub async fn send(&self, channel_id: &str, data: impl Into<CreateMessageData>) {
        let data = data.into();
        let body = data.serialize_json();

        let post_url = format!("{api}/channels/{channel_id}/messages", api = consts::API);

        self.send_req(&post_url, body)
            .await
            .expect("Failed to send http request");
    }

    pub async fn reply(&self, reply_to: &MessageData, data: impl Into<CreateMessageData>) {
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

        self.send_req(&post_url, json::stringify(body))
            .await
            .expect("Failed to send http request");
    }

    pub async fn get_channel(
        &self,
        channel_id: &str,
    ) -> Result<Channel, Box<dyn std::error::Error>> {
        let post_url = &format!("{api}/channels/{channel_id}", api = consts::API);

        let resp = reqwest::Client::new()
            .get(post_url)
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bot {}", self.token))
            .send()
            .await?;

        Ok(Channel::deserialize_json(&resp.text().await?)?)
    }

    async fn send_req(&self, url: &str, body: String) -> reqwest::Result<Response> {
        reqwest::Client::new()
            .post(url)
            .body(body)
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bot {}", self.token))
            .send()
            .await
    }
}
