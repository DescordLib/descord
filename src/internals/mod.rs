use std::collections::HashMap;

mod commands;
mod components;
mod events;
mod slash_commands;

pub use commands::*;
pub use components::*;
pub use events::*;
pub use slash_commands::*;

use crate::consts::events::Event;
use crate::models::channel::Channel;
use crate::models::deleted_message_response::DeletedMessage;
use crate::models::interaction::{Interaction, InteractionData};
use crate::models::misc::Reconnect;
use crate::models::reaction_response::Reaction;
use crate::prelude::*;
use crate::utils::*;
use futures_util::FutureExt;

use thiserror::Error;

pub type HandlerFn = fn(
    Message,
    Vec<Value>,
) -> std::pin::Pin<
    Box<dyn futures_util::Future<Output = DescordResult> + Send + 'static>,
>;

#[derive(Error, Debug)]
pub enum DescordError {
    #[error("Missing required argument for command: {0}")]
    MissingRequiredArgument(String),
}

#[macro_export]
macro_rules! implemented_enum {
    [ $vis:vis enum $name:ident { $($variant:ident),* $(,)? } ] => {
        #[derive(Debug, Clone)]
        $vis enum $name {
            $($variant($variant),)*
        }

        $(
            impl From<$variant> for $name {
                fn from(value: $variant) -> Self {
                    $name::$variant(value)
                }
            }
        )*
    };
}

/// Paramter type info (meant to be used in attribute macro).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    String,
    Int,
    Bool,
    Channel,
    User,
    Args,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Int(isize),
    Bool(bool),
    Channel(Channel),
    User(User),
    Args(Vec<String>),

    StringOption(Option<String>),
    IntOption(Option<isize>),
    BoolOption(Option<bool>),
    ChannelOption(Option<Channel>),
    UserOption(Option<User>),

    None,
}

fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut quote_char = None;
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' if quote_char.is_none() => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            '\'' | '"' => {
                if quote_char.is_none() {
                    quote_char = Some(c);
                    current_arg.push(c);
                } else if quote_char.unwrap() == c {
                    quote_char = None;
                    current_arg.remove(0);
                } else {
                    current_arg.push(c);
                }
            }
            _ => current_arg.push(c),
        }
    }

    if !current_arg.is_empty() {
        if quote_char.is_some() {
            args.extend(current_arg.split_whitespace().map(|s| s.to_string()));
        } else {
            args.push(current_arg);
        }
    }

    args
}
