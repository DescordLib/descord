use std::collections::HashMap;

use crate::handlers::events::Event;
use crate::models::channel::Channel;
use crate::models::deleted_message_response::DeletedMessage;
use crate::models::reaction_response::Reaction;
use crate::prelude::*;
use crate::utils::*;
use futures_util::FutureExt;

macro_rules! implemented_enum {
    [ $vis:vis enum $name:ident { $($variant:ident),* $(,)? } ] => {
        #[derive(Debug, Clone)]
        $vis enum $name {
            $($variant($variant),)*
        }

        $(
            impl From<$variant> for $name {
                fn from(value: $variant) -> Self {
                    HandlerValue::$variant(value)
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
}

implemented_enum! {
    pub enum HandlerValue {
        ReadyData,
        Message,
        DeletedMessage,
        Reaction,
        GuildCreate,
    }
}

pub type HandlerFn =
    fn(
        Message,
        Vec<Value>,
    ) -> std::pin::Pin<Box<dyn futures_util::Future<Output = ()> + Send + 'static>>;

pub type EventHandlerFn =
    fn(HandlerValue) -> std::pin::Pin<Box<dyn futures_util::Future<Output = ()> + Send + 'static>>;

#[derive(Debug, Clone)]
pub struct EventHandler {
    pub event: Event,
    pub handler_fn: EventHandlerFn,
}

impl EventHandler {
    pub async fn call(&self, data: HandlerValue) {
        let fut = ((self.handler_fn)(data));
        let boxed_fut: std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> =
            Box::pin(fut);
        boxed_fut.await;
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub custom_prefix: bool,
    pub fn_sig: Vec<ParamType>,
    pub handler_fn: HandlerFn,
}

impl Command {
    pub async fn call(&self, data: Message) {
        let split = data.content.split_whitespace().collect::<Vec<_>>();

        // -1 because of the command name
        assert!(
            split.len() - 1 == self.fn_sig.len() || self.fn_sig.last() == Some(&ParamType::Args)
        );

        // check if this command is really called
        assert_eq!(split[0], self.name);

        let mut args: Vec<Value> = Vec::with_capacity(self.fn_sig.len());

        let mut idx = 1;
        while idx - 1 < self.fn_sig.len() {
            let ty = &self.fn_sig[idx - 1];
            match ty {
                ParamType::String => args.push(Value::String((split[idx].to_owned()))),
                ParamType::Int => args.push(Value::Int(split[idx].parse::<isize>().unwrap())),
                ParamType::Bool => args.push(Value::Bool(split[idx].parse::<bool>().unwrap())),
                ParamType::Channel => {
                    let channel_id_str = split[idx];
                    let channel_id =
                        if channel_id_str.starts_with("<#") && channel_id_str.ends_with(">") {
                            &channel_id_str[2..channel_id_str.len() - 1]
                        } else {
                            channel_id_str
                        };
                    args.push(Value::Channel(get_channel(channel_id).await.unwrap()));
                }

                ParamType::User => {
                    let user_id_str = split[idx];
                    let user_id = if user_id_str.starts_with("<@") && user_id_str.ends_with(">") {
                        &user_id_str[2..user_id_str.len() - 1]
                    } else {
                        user_id_str
                    };
                    args.push(Value::User(get_user(user_id).await.unwrap()));
                }

                ParamType::Args => {
                    args.push(Value::Args(
                        split[idx..].iter().map(|i| i.to_string()).collect(),
                    ));
                }
            }

            idx += 1;
        }

        // if no extra args, send an empty vector
        if args.len() != self.fn_sig.len() && self.fn_sig.last() == Some(&ParamType::Args) {
            args.push(Value::Args(vec![]));
        }

        let fut = ((self.handler_fn)(data, args));
        let boxed_fut: std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> =
            Box::pin(fut);
        boxed_fut.await;
    }
}
