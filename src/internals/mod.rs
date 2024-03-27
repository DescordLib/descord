use std::collections::HashMap;

use crate::handlers::events::Event;
use crate::models::channel::Channel;
use crate::models::deleted_message_response::DeletedMessage;
use crate::models::interaction::{Interaction, InteractionData};
use crate::models::reaction_response::Reaction;
use crate::prelude::*;
use crate::utils::*;
use futures_util::FutureExt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DescordError {
    #[error("Missing required argument for command: {0}")]
    MissingRequiredArgument(String),
}

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

    StringOption(Option<String>),
    IntOption(Option<isize>),
    BoolOption(Option<bool>),
    ChannelOption(Option<Channel>),
    UserOption(Option<User>),

    None,
}

implemented_enum! {
    pub enum HandlerValue {
        ReadyData,
        Message,
        DeletedMessage,
        Reaction,
        GuildCreate,
        Interaction,
    }
}

pub type HandlerFn = fn(
    Message,
    Vec<Value>,
) -> std::pin::Pin<
    Box<dyn futures_util::Future<Output = DescordResult> + Send + 'static>,
>;

pub type SlashHandlerFn = fn(
    Interaction,
    Vec<Value>,
) -> std::pin::Pin<
    Box<dyn futures_util::Future<Output = DescordResult> + Send + 'static>,
>;

pub type EventHandlerFn = fn(
    HandlerValue,
) -> std::pin::Pin<
    Box<dyn futures_util::Future<Output = DescordResult> + Send + 'static>,
>;

#[derive(Debug, Clone)]
pub struct EventHandler {
    pub event: Event,
    pub handler_fn: EventHandlerFn,
}

impl EventHandler {
    pub async fn call(&self, data: HandlerValue) -> DescordResult {
        let fut = ((self.handler_fn)(data));
        let boxed_fut: std::pin::Pin<
            Box<
                dyn std::future::Future<Output = DescordResult>
                    + Send
                    + 'static,
            >,
        > = Box::pin(fut);
        boxed_fut.await
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub custom_prefix: bool,
    pub fn_sig: Vec<ParamType>,
    pub handler_fn: HandlerFn,
    pub optional_params: Vec<bool>,
}

impl Command {
    pub async fn call(&self, data: Message) -> DescordResult {
        let re = regex::Regex::new(r#"([^"\s']+)|"([^"]*)"|'([^']*)'"#).unwrap();
        let split: Vec<String> = re
            .captures_iter(&data.content)
            .filter_map(|cap| cap.get(1).or(cap.get(2)).or(cap.get(3)))
            .map(|m| m.as_str().to_string())
            .collect();

        let mut args: Vec<Value> = Vec::with_capacity(self.fn_sig.len());

        let mut idx = 1;
        while idx - 1 < self.fn_sig.len() {
            let ty = &self.fn_sig[idx - 1];
            let optional = self.optional_params[idx - 1];
            if idx < split.len() {
                match ty {
                    ParamType::String => args.push(if optional {
                        Value::StringOption(Some(split[idx].to_owned()))
                    } else {
                        Value::String(split[idx].to_owned())
                    }),
                    ParamType::Int => args.push(if optional {
                        Value::IntOption(Some(split[idx].parse::<isize>().unwrap()))
                    } else {
                        Value::Int(split[idx].parse::<isize>().unwrap())
                    }),
                    ParamType::Bool => args.push(if optional {
                        Value::BoolOption(Some(split[idx].parse::<bool>().unwrap()))
                    } else {
                        Value::Bool(split[idx].parse::<bool>().unwrap())
                    }),
                    ParamType::Channel => {
                        let channel_id_str = &split[idx];
                        let channel_id =
                            if channel_id_str.starts_with("<#") && channel_id_str.ends_with(">") {
                                &channel_id_str[2..channel_id_str.len() - 1]
                            } else {
                                channel_id_str
                            };
                        match get_channel(channel_id).await {
                            Ok(channel) => args.push(if optional {
                                Value::ChannelOption(Some(channel))
                            } else {
                                Value::Channel(channel)
                            }),
                            Err(_) => {
                                if !optional {
                                    panic!("Channel not found")
                                }
                            }
                        }
                    }
                    ParamType::User => {
                        let user_id_str = &split[idx];
                        let user_id = if user_id_str.starts_with("<@") && user_id_str.ends_with(">")
                        {
                            &user_id_str[2..user_id_str.len() - 1]
                        } else {
                            user_id_str
                        };
                        match get_user(user_id).await {
                            Ok(user) => args.push(if optional {
                                Value::UserOption(Some(user))
                            } else {
                                Value::User(user)
                            }),
                            Err(_) => {
                                if !optional {
                                    panic!("User not found")
                                }
                            }
                        }
                    }
                    ParamType::Args => {
                        args.push(Value::Args(
                            split[idx..].iter().map(|i| i.to_string()).collect(),
                        ));
                    }
                    _ => {}
                }
            } else if optional {
                match ty {
                    ParamType::String => args.push(Value::StringOption(None)),
                    ParamType::Int => args.push(Value::IntOption(None)),
                    ParamType::Bool => args.push(Value::BoolOption(None)),
                    ParamType::Channel => args.push(Value::ChannelOption(None)),
                    ParamType::User => args.push(Value::UserOption(None)),
                    _ => {}
                }
            } else {
                return Err(Box::new(DescordError::MissingRequiredArgument(
                    self.name.clone(),
                )));
            }

            idx += 1;
        }

        let fut = ((self.handler_fn)(data, args));
        let boxed_fut: std::pin::Pin<
            Box<
                dyn std::future::Future<Output = DescordResult>
                    + Send
                    + 'static,
            >,
        > = Box::pin(fut);

        boxed_fut.await?;

        Ok(())
    }
}

pub(crate) type AutoCompleteFn =
    fn(
        String,
    ) -> std::pin::Pin<Box<dyn futures_util::Future<Output = Vec<String>> + Send + 'static>>;

#[derive(Debug, Clone)]
pub struct SlashCommand {
    pub name: String,
    pub description: String,
    pub fn_sig: Vec<ParamType>,
    pub handler_fn: SlashHandlerFn,
    pub fn_param_names: Vec<String>,
    pub fn_param_descriptions: Vec<String>,
    pub optional_params: Vec<bool>,
    pub fn_param_renames: Vec<Option<String>>,
    pub fn_param_autocomplete: Vec<Option<AutoCompleteFn>>,
}

impl SlashCommand {
    pub async fn call(&self, data: Interaction) -> DescordResult {
        let split: Vec<String> = data
            .clone()
            .data
            .unwrap_or(InteractionData::default())
            .options
            .unwrap_or_default()
            .iter()
            .map(|i| i.value.clone())
            .collect();
        let mut args: Vec<Value> = Vec::with_capacity(self.fn_sig.len());

        let mut idx = 0;
        while idx < self.fn_sig.len() {
            let ty = &self.fn_sig[idx];
            let optional = self.optional_params[idx];
            if idx < split.len() {
                match ty {
                    ParamType::String => args.push(if optional {
                        Value::StringOption(Some(split[idx].to_owned()))
                    } else {
                        Value::String(split[idx].to_owned())
                    }),
                    ParamType::Int => args.push(if optional {
                        Value::IntOption(Some(split[idx].parse::<isize>().unwrap()))
                    } else {
                        Value::Int(split[idx].parse::<isize>().unwrap())
                    }),
                    ParamType::Bool => args.push(if optional {
                        Value::BoolOption(Some(split[idx].parse::<bool>().unwrap()))
                    } else {
                        Value::Bool(split[idx].parse::<bool>().unwrap())
                    }),
                    ParamType::Channel => {
                        let channel_id_str = &split[idx];
                        let channel_id =
                            if channel_id_str.starts_with("<#") && channel_id_str.ends_with(">") {
                                &channel_id_str[2..channel_id_str.len() - 1]
                            } else {
                                channel_id_str
                            };
                        match get_channel(channel_id).await {
                            Ok(channel) => args.push(if optional {
                                Value::ChannelOption(Some(channel))
                            } else {
                                Value::Channel(channel)
                            }),
                            Err(e) => {
                                log::error!("{:?}", e);
                                if !optional {
                                    panic!("Channel not found")
                                }
                            }
                        }
                    }
                    ParamType::User => {
                        let user_id_str = &split[idx];
                        let user_id = if user_id_str.starts_with("<@") && user_id_str.ends_with(">")
                        {
                            &user_id_str[2..user_id_str.len() - 1]
                        } else {
                            user_id_str
                        };
                        match get_user(user_id).await {
                            Ok(user) => args.push(if optional {
                                Value::UserOption(Some(user))
                            } else {
                                Value::User(user)
                            }),
                            Err(e) => {
                                log::error!("{:?}", e);
                                if !optional {
                                    panic!("User not found")
                                }
                            }
                        }
                    }
                    _ => {}
                }
            } else if optional {
                match ty {
                    ParamType::String => args.push(Value::StringOption(None)),
                    ParamType::Int => args.push(Value::IntOption(None)),
                    ParamType::Bool => args.push(Value::BoolOption(None)),
                    ParamType::Channel => args.push(Value::ChannelOption(None)),
                    ParamType::User => args.push(Value::UserOption(None)),
                    _ => {}
                }
            } else {
                return Err(Box::new(DescordError::MissingRequiredArgument(
                    self.name.clone(),
                )));
            }

            idx += 1;
        }

        let fut = ((self.handler_fn)(data, args));
        let boxed_fut: std::pin::Pin<
            Box<
                dyn std::future::Future<Output = DescordResult>
                    + Send
                    + 'static,
            >,
        > = Box::pin(fut);

        boxed_fut.await?;
        Ok(())
    }
}
