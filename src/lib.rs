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

pub(crate) mod cache;

pub mod prelude {
    pub use super::*;
    pub use super::{
        color::*,
        consts::ImageFormat,
        consts::ButtonStyle,
        consts::ComponentType,
        consts::SelectMenuType,
        intents::GatewayIntent,
        models::{
            channel::*, channel::*, component_builder::*, components::*, embed::*,
            embed_builder::*, guild::*, message_edit::MessageEditData,
            message_response::CreateMessageData, message_response::Message,
            reaction_response::Reaction, ready_response::*, user::User,
        },
        Payload,
    };
}
