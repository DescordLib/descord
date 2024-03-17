use crate::consts;
use crate::models::message_response::CreateMessageData;
use crate::prelude::MessageData;

use json::object;
use nanoserde::SerJson;

pub struct Context {
    token: String,
}

impl Context {
    pub(crate) fn new(token: String) -> Self {
        Self {
            token: token.into(),
        }
    }

    pub async fn send(&self, channel_id: impl AsRef<str>, data: impl Into<CreateMessageData>) {
        let data = data.into();
        let body = data.serialize_json();

        let post_url = &format!(
            "{api}/channels/{channel_id}/messages",
            api = consts::API,
            channel_id = channel_id.as_ref()
        );

        reqwest::Client::new()
            .post(post_url)
            .body(body)
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bot {}", self.token))
            .send()
            .await
            .expect("Failed to send http request");
    }

    pub async fn reply(&self, reply_to: &MessageData, data: impl Into<CreateMessageData>) {
        let data = data.into();
        let body = json::stringify(object! {
            content: data.content,
            tts: data.tts,
            message_reference: {
                message_id: reply_to.message_id.as_str(),
                guild_id: reply_to.guild_id.as_str(),
            }
        });

        let post_url = &format!(
            "{api}/channels/{channel_id}/messages",
            api = consts::API,
            channel_id = reply_to.channel_id
        );

        reqwest::Client::new()
            .post(post_url)
            .body(body)
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bot {}", self.token))
            .send()
            .await
            .expect("Failed to send http request");
    }
}
