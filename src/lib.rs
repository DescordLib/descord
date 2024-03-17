#![allow(unused)]

mod client;
mod consts;
mod handlers;
mod models;
mod ws;

pub use async_trait::async_trait;

pub use client::Client;
pub use consts::intents;
pub use handlers::EventHandler;
pub use ws::payload::Payload;

pub mod prelude {
    pub use super::{
        intents::GatewayIntent,
        models::{guild::Guild, message_response::MessageData, ready_response::*, user::User, message_response::CreateMessageData},
        Client, EventHandler, Payload,
        client::Context,
        async_trait,
    };
}
