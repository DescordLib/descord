#![allow(unused)]

mod client;
mod consts;
mod handlers;
pub mod models;
mod ws;

pub use async_trait::async_trait;

pub use client::Client;
pub use consts::intents;
pub use consts::color;
pub use handlers::EventHandler;
pub use ws::payload::Payload;

pub mod prelude {
    pub use super::{
        async_trait,
        client::Context,
        intents::GatewayIntent,
        models::{
            embed::*, embed_builder::EmbedBuilder, guild::Guild,
            message_response::CreateMessageData, message_response::MessageData, ready_response::*,
            user::User,
        },
        color::*,
        Client, EventHandler, Payload,
    };
}
