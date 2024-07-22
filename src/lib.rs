//! Descord is a minimal and easy to use discord api wrapper
#![allow(unused)]

mod client;
mod consts;
mod handlers;
mod ws;

pub mod models;

pub mod utils;
pub use client::Client;
pub use consts::color;
pub use consts::intents;
pub use descord_macros::*;
pub use handlers::events::Event;
pub use ws::payload::Payload;
pub mod internals;

// TODO: change the error type
pub type DescordResult = Result<(), Box<dyn std::error::Error + Send>>;

pub(crate) mod cache;

pub mod prelude {
    pub use super::*;
    pub use super::{
        color::*,
        consts::ButtonStyle,
        consts::ComponentType,
        consts::ImageFormat,
        consts::SelectMenuType,
        intents::GatewayIntent,
        models::{
            channel::*, channel::*, component_builder::*, components::*, embed::*,
            embed_builder::*, guild::*, interaction::*, message_edit::MessageEditData,
            message_response::CreateMessageData, message_response::Message,
            reaction_response::Reaction, ready_response::*, role::Role, role_response::*,
            user::User,
        },
        Payload,
    };
}
