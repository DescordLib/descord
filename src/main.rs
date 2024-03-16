#![allow(unused)]

use descord::*;
use descord::intents::GatewayIntent;

fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let mut client = Client::new(
        &std::env::var("DISCORD_TOKEN").unwrap(),
        GatewayIntent::MessageContent | GatewayIntent::GuildMessages,
    );

    client.login(Handler);
}

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, payload: Payload) {
        let username = payload.data["user"]["username"].as_str().unwrap();
        let discriminator = payload.data["user"]["discriminator"].as_str().unwrap();

        println!("Logged in as: {username}#{discriminator}",);
    }

    fn message_create(&self, payload: Payload) {
        let author = payload.data["author"]["global_name"].as_str().unwrap();
        let content = payload.data["content"].as_str().unwrap();

        println!("Message received from `{author}`, message: '{content}'",)
    }
}

/*
{
    "t": "MESSAGE_CREATE",
    "s": 2,
    "op": 0,
    "d": {
        "type": 0,
        "tts": false,
        "timestamp": "2024-03-16T06:04:56.199000+00:00",
        "referenced_message": null,
        "pinned": false,
        "nonce": "1218439840215859200",
        "mentions": [],
        "mention_roles": [],
        "mention_everyone": false,
        "member": {
            "roles": [
                "1218051460751687770"
            ],
            "premium_since": null,
            "pending": false,
            "nick": null,
            "mute": false,
            "joined_at": "2024-03-15T03:27:11.652000+00:00",
            "flags": 0,
            "deaf": false,
            "communication_disabled_until": null,
            "avatar": null
        },
        "id": "1218439843462254682",
        "flags": 0,
        "embeds": [],
        "edited_timestamp": null,
        "content": "",
        "components": [],
        "channel_id": "1204212041158492164",
        "author": {
            "username": "_thatmagicalcat",
            "public_flags": 0,
            "premium_type": 0,
            "id": "815189874478546954",
            "global_name": "thatmagicalcat",
            "discriminator": "0",
            "avatar_decoration_data": null,
            "avatar": "6a7a25edccdcabbe9f29558a0b6a7f00"
        },
        "attachments": [],
        "guild_id": "1204212039896277093"
    }
}
*/

/*
Payload: {
    "t": "READY",
    "s": 1,
    "op": 0,
    "d": {
    "v": 10,
    "user_settings": {},
    "user": {
    "verified": true,
    "username": "test",
    "mfa_enabled": false,
    "id": "1218041224938651739",
    "global_name": null,
    "flags": 0,
    "email": null,
    "discriminator": "0839",
    "bot": true,
    "avatar": null
    },
    "session_type": "normal",
    "session_id": "57dc137d881caef544d7b9a7024a788d",
    "resume_gateway_url": "wss://gateway-us-east1-b.discord.gg",
    "relationships": [],
    "private_channels": [],
    "presences": [],
    "guilds": [
    {
    "unavailable": true,
    "id": "1204212039896277093"
    }
    ],
    "guild_join_requests": [],
    "geo_ordered_rtc_regions": [
    "india",
    "dubai",
    "hongkong",
    "singapore",
    "tel-aviv"
    ],
    "auth": {},
    "application": {
    "id": "1218041224938651739",
    "flags": 16777216
*/
