use super::*;

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

pub async fn update_rate_limit_info(headers: &HeaderMap<HeaderValue>, bucket: &str) {
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

pub fn get_headers() -> HeaderMap {
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
