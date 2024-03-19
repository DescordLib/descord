#![allow(unused)]

mod client;
mod consts;
mod handlers;
pub mod models;
mod ws;

pub mod utils;

pub use async_trait::async_trait;

pub use client::Client;
pub use consts::color;
pub use consts::intents;
pub use handlers::EventHandler;
pub use macros::*;
pub use ws::payload::Payload;

pub mod internals;

pub mod prelude {
    pub use super::*;
    pub use super::{
        color::*,
        intents::GatewayIntent,
        models::{
            embed::*, embed_builder::EmbedBuilder, guild::Guild, message_edit::MessageEditData,
            message_response::CreateMessageData, message_response::MessageData, ready_response::*,
            user::User,
        },
        Payload,
    };
}
