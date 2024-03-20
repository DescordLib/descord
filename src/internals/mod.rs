use std::collections::HashMap;

use crate::handlers::events::Event;
use crate::models::channel::Channel;
use crate::models::deleted_message_response::DeletedMessageData;
use crate::models::reaction_response::Reaction;
use crate::prelude::*;
use crate::utils::*;
use futures_util::FutureExt;

/// Paramter type info (meant to be used in attribute macro).
#[derive(Debug, Clone, Copy)]
pub enum ParamType {
    String,
    Int,
    Bool,
    Channel,
    User,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Int(isize),
    Bool(bool),
    Channel(Channel),
    User(User),
}

#[derive(Debug, Clone)]
pub enum HandlerValue {
    ReadyData(ReadyData),
    MessageData(Message),
    DeletedMessageData(DeletedMessageData),
    ReactionData(Reaction),
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
    pub args: Vec<ParamType>,
    pub handler_fn: HandlerFn,
}

impl Command {
    pub async fn call(&self, data: Message) {
        let split = data.content.split_whitespace().collect::<Vec<_>>();

        // -1 because of the command name
        assert_eq!(split.len() - 1, self.args.len());

        // check if this command is really called
        assert_eq!(split[0], self.name);

        let mut args: Vec<Value> = Vec::with_capacity(self.args.len());

        for (idx, ty) in self.args.iter().enumerate() {
            match ty {
                ParamType::String => args.push(Value::String((split[idx + 1].to_owned()))),
                ParamType::Int => args.push(Value::Int(split[idx + 1].parse::<isize>().unwrap())),
                ParamType::Bool => args.push(Value::Bool(split[idx + 1].parse::<bool>().unwrap())),
                ParamType::Channel => {
                    let channel_id_str = split[idx + 1];
                    let channel_id =
                        if channel_id_str.starts_with("<#") && channel_id_str.ends_with(">") {
                            &channel_id_str[2..channel_id_str.len() - 1]
                        } else {
                            channel_id_str
                        };
                    args.push(Value::Channel(get_channel(channel_id).await.unwrap()));
                }
                ParamType::User => {
                    let user_id_str = split[idx + 1];
                    let user_id = if user_id_str.starts_with("<@") && user_id_str.ends_with(">") {
                        &user_id_str[2..user_id_str.len() - 1]
                    } else {
                        user_id_str
                    };
                    args.push(Value::User(get_user(user_id).await.unwrap()));
                }
                _ => {}
            }
        }

        let fut = ((self.handler_fn)(data, args));
        let boxed_fut: std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> =
            Box::pin(fut);
        boxed_fut.await;
    }
}
