//! Descord is a minimal and easy to use discord api wrapper.
//!
//! # Example
//! ```rust
//! use descord::prelude::*;

//! #[tokio::main]
//! async fn main() {
//!     let mut client = Client::new(
//!         "TOKEN",
//!         GatewayIntent::NON_PRIVILEGED
//!             // for message commands
//!             | GatewayIntent::MESSAGE_CONTENT,
//!         "!", // default prefix for message command
//!     )
//!     .await;
//!     
//!     // register commands, events and slash commands manually
//!     client.register_commands(vec![echo()]);
//!     client.register_events(vec![ready()]);
//!     client.register_slash_commands(vec![avatar()]).await;
//!
//!
//!     // alternatively you can do this, this is neat but
//!     // it is very counterintuitive since it read
//!     // the files and find functions marked with
//!     // proc macros
//!
//!     // register_all!(client => ["src/main.rs", "src/file1.rs"]);
//!
//!
//!     // start the bot!
//!     client.login().await;
//! }
//!
//! // An event handler
//! #[descord::event] // Alternatively you can do `#[descord::event(ready)]`
//! async fn ready(data: ReadyData) {
//!     println!(
//!         "Logged in as {}#{}",
//!         data.user.username, data.user.discriminator
//!     )
//! }
//!
//! // A message command
//! //
//! // you can also do `#[descord::command(prefix = "new_prefix")]` to change
//! // the command prefix for this command to change
//! // the command prefix for this command
//! #[descord::command]
//! async fn echo(
//!     /// information about the messgae
//!     msg: Message,
//!     /// some types can be parsed automatically
//!     echo_what: String,
//! ) {
//!     msg.reply(echo_what).await;
//! }
//!
//! // A slash command
//! #[descord::slash(description = "Get a user's avatar")]
//! async fn avatar(
//!     interaction: Interaction,
//!     #[doc = "User to fetch avatar from"] user: Option<User>,
//! ) {
//!     let member = interaction.member.as_ref().unwrap();
//!     let (username, avatar_url) = match user {
//!         Some(user) => (
//!             &user.username,
//!             user.get_avatar_url(ImageFormat::WebP, None).unwrap(),
//!         ),
//!
//!         _ => (
//!             &member.user.as_ref().unwrap().username,
//!             member.get_avatar_url(ImageFormat::WebP, None).unwrap(),
//!         ),
//!     };
//!
//!     // Creating an embed
//!     let embed = EmbedBuilder::new()
//!         .color(Color::Orange)
//!         .title(&format!("{username}'s avatar"))
//!         .image(avatar_url, None, None)
//!         .build();
//!
//!     interaction.reply(embed, false).await;
//! }
//! ```

#![allow(unused)]

mod client;
mod consts;
mod ws;

pub mod models;

pub mod utils;
pub use client::Client;
pub use consts::color;
pub use consts::intents;
pub use descord_macros::*;
pub use consts::events::Event;
pub use ws::payload::Payload;
pub mod internals;

// TODO: change the error type
pub type DescordResult = Result<(), Box<dyn std::error::Error + Send>>;

pub(crate) mod cache;

pub mod prelude {
    pub use super::*;
    pub use super::{
        color::*,
        consts::permissions,
        consts::ButtonStyle,
        consts::ComponentType,
        consts::ImageFormat,
        consts::SelectMenuType,
        intents::GatewayIntent,
        models::{
            channel::*, channel::*, component_builder::*, components::*, embed::*,
            embed_builder::*, guild::*, interaction::*, message_response::CreateMessageData,
            message_response::Message, reaction_response::Reaction, ready_response::*, role::Role,
            role_response::*, user::User,
        },
        Payload,
    };
}
