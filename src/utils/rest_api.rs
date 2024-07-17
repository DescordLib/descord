use crate::cache::{RateLimitInfo, ENDPOINT_BUCKET_MAP, MESSAGE_CACHE, RATE_LIMITS, ROLE_CACHE};
use crate::client::TOKEN;
use crate::consts::API;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::models::application_command::ApplicationCommand;
use crate::models::channel::Channel;
use crate::models::dm_channel::DirectMessageChannel;
use crate::models::message_edit::MessageEditData;
use crate::models::message_response::CreateMessageData;

use crate::prelude::{Guild, Member, Message};
use crate::prelude::{Role, User};

use futures_util::TryFutureExt;
use json::{object, JsonValue};
use log::info;
use nanoserde::{DeJson, SerJson};
use reqwest::header::HeaderValue;
use reqwest::{header::HeaderMap, Client, Error, Method, Response, StatusCode};
use tokio::time::sleep;

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

pub async fn fetch_bot_id() -> String {
    let response = request(Method::GET, "users/@me", None).await;
    json::parse(response.text().await.unwrap().as_str()).unwrap_or_else(|_| {
        log::error!("Failed to parse JSON response");
        JsonValue::Null
    })["id"]
        .as_str()
        .unwrap_or_else(|| {
            log::error!("Failed to get 'id' from JSON response");
            ""
        })
        .to_string()
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

pub async fn fetch_guild(guild_id: &str) -> Result<Guild, Box<dyn std::error::Error>> {
    let url = format!("guilds/{guild_id}");
    let resp = request(Method::GET, &url, None).await.text().await?;
    Guild::deserialize_json(&resp).map_err(|e| e.into())
}

pub async fn fetch_channel(channel_id: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let url = format!("channels/{channel_id}");
    let resp = request(Method::GET, &url, None).await.text().await?;
    let mut channel = Channel::deserialize_json(&resp).map_err(|e| e.into());
    if let Ok(channel) = &mut channel {
        channel.mention = format!("<#{}>", channel.id);
    }
    channel
}

pub async fn fetch_user(user_id: &str) -> Result<User, Box<dyn std::error::Error>> {
    let url = format!("users/{}", user_id);
    let resp = request(Method::GET, &url, None).await;
    let mut user = User::deserialize_json(&resp.text().await?)?;
    user.mention = format!("<@{}>", user.id);
    Ok(user)
}

pub async fn fetch_member(
    guild_id: &str,
    user_id: &str,
) -> Result<Member, Box<dyn std::error::Error>> {
    let url = format!("guilds/{guild_id}/members/{user_id}");
    let resp = request(Method::GET, &url, None).await;
    let mut member = Member::deserialize_json(&resp.text().await?)?;
    member.mention = format!("<@{}>", user_id);
    Ok(member)
}

/// Returns true if the operation was successful, false otherwise.
/// This function requires the MANAGE_MESSAGES permission.
pub async fn delete_message(channel_id: &str, message_id: &str) -> bool {
    let url = format!("channels/{channel_id}/messages/{message_id}");

    let resp = request(Method::DELETE, &url, None).await;
    resp.status() == StatusCode::NO_CONTENT
}

pub async fn edit_message(channel_id: &str, message_id: &str, data: impl Into<MessageEditData>) {
    let url = format!("channels/{channel_id}/messages/{message_id}");
    let data: MessageEditData = data.into();

    request(
        Method::PATCH,
        &url,
        Some(json::parse(&data.serialize_json()).unwrap()),
    )
    .await;
}

/// Returns a new DM channel with a user (or return
/// an existing one). Returns a `DirectMessageChannel` object.
pub async fn fetch_dm(user_id: &str) -> DirectMessageChannel {
    let url = format!("users/@me/channels");
    let data = json::stringify(object! {
        recipient_id: user_id
    });

    let response = request(Method::POST, &url, Some(json::parse(&data).unwrap())).await;
    DirectMessageChannel::deserialize_json(&response.text().await.unwrap()).unwrap()
}

pub async fn send_dm(user_id: &str, data: impl Into<CreateMessageData>) {
    let dm_channel = fetch_dm(user_id).await;
    send(&dm_channel.id, data).await;
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

pub async fn fetch_role(guild_id: &str, role_id: &str) -> Result<Role, Box<dyn std::error::Error>> {
    if let Some(role) = ROLE_CACHE.lock().await.get(role_id).cloned() {
        info!("Role cache hit");
        return Ok(role);
    }
    let url = format!("guilds/{guild_id}/roles");
    let resp = request(Method::GET, &url, None).await.text().await.unwrap();
    let roles: Vec<Role> = DeJson::deserialize_json(&resp).unwrap();
    let mut answer = None;
    for role in &roles {
        ROLE_CACHE.lock().await.put(role.id.clone(), role.clone());
        if role.id == role_id {
            answer = Some(role.clone());
        }
    }
    if let Some(answer) = answer {
        Ok(answer)
    } else {
        Err("Role not found".into())
    }
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

async fn update_rate_limit_info(headers: &HeaderMap<HeaderValue>, bucket: &str) {
    let remaining = headers
        .get("x-ratelimit-remaining")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let reset = headers
        .get("x-ratelimit-reset")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0);

    let rate_limit_info = RateLimitInfo { remaining, reset };

    RATE_LIMITS
        .lock()
        .await
        .put(bucket.to_string(), rate_limit_info);
}

async fn wait_for_rate_limit(bucket: &str) {
    if let Some(rate_limit_info) = RATE_LIMITS.lock().await.get(bucket) {
        log::info!("Rate limit hit: {:?}", rate_limit_info);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        if rate_limit_info.remaining == 0 && rate_limit_info.reset > now {
            let delay = Duration::from_secs_f64(rate_limit_info.reset - now);
            sleep(delay).await;
        }
    }
}

pub async fn request(method: Method, endpoint: &str, data: Option<JsonValue>) -> Response {
    let client = Client::new();
    let url = format!("{}/{}", API, endpoint);

    let mut request_builder = client.request(method, &url);
    request_builder = request_builder.headers(get_headers());

    if let Some(body) = data {
        request_builder = request_builder.body(body.to_string());
    }

    let bucket = ENDPOINT_BUCKET_MAP.lock().await.get(endpoint).cloned();
    let seen;
    if let Some(bucket) = bucket {
        wait_for_rate_limit(&bucket).await;
        seen = true;
    } else {
        seen = false;
    }

    let mut response = request_builder.try_clone().unwrap().send().await.unwrap();
    while response.status() == StatusCode::TOO_MANY_REQUESTS {
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);
        log::warn!(
            "Rate limited on endpoint: {}, retrying after {} seconds",
            endpoint,
            retry_after
        );
        sleep(Duration::from_secs_f32(retry_after)).await;
        response = request_builder.try_clone().unwrap().send().await.unwrap();
    }

    if let Some(bucket) = response.headers().get("x-ratelimit-bucket") {
        let bucket = bucket.to_str().unwrap_or_default();
        update_rate_limit_info(response.headers(), bucket).await;
        if !seen {
            ENDPOINT_BUCKET_MAP
                .lock()
                .await
                .put(endpoint.to_string(), bucket.to_string());
        }
    }

    response
}
