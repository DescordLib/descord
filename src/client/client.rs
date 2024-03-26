use reqwest::Method;
use std::collections::HashMap;

use std::sync::Mutex;

use json::object;
use nanoserde::SerJson;

use crate::consts::intents::GatewayIntent;
use crate::internals::{EventHandler, *};
use crate::models::application_command::ApplicationCommandOption;
use crate::prelude::{CreateMessageData, Message};
use crate::utils::{self, send_request};
use crate::ws::WsManager;
use crate::{consts, internals, Event};

use log::{error, info};

// SAFETY: These will always be valid if accessed from an event.
lazy_static::lazy_static! {
    pub(crate) static ref BOT_ID: Mutex<Option<String>> = Mutex::new(None);
    pub(crate) static ref TOKEN: Mutex<Option<String>> = Mutex::new(None);
}

pub struct Client {
    intents: u32,
    ws: WsManager,
    token: String,
    commands: HashMap<String, Command>,
    slash_commands: HashMap<String, SlashCommand>,
    event_handlers: HashMap<Event, EventHandler>,
    prefix: String,
}

impl Client {
    pub async fn new(token: &str, intents: impl Into<u32>, prefix: &str) -> Self {
        *TOKEN.lock().unwrap() = Some(token.to_owned());

        Self {
            intents: intents.into(),
            token: token.to_owned(),
            ws: WsManager::new(token)
                .await
                .expect("Failed to initialize websockets"),
            prefix: prefix.to_owned(),

            commands: HashMap::new(),
            slash_commands: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }

    pub async fn login(self) {
        self.ws
            .connect(
                self.intents,
                self.event_handlers.into(),
                self.commands.into(),
                self.slash_commands.into(),
            )
            .await;
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn register_events(&mut self, events: Vec<EventHandler>) {
        events.into_iter().for_each(|event| {
            if self.event_handlers.contains_key(&event.event) {
                panic!("{:?} is already hooked", event.event);
            }

            self.event_handlers.insert(event.event, event);
        });
    }

    pub fn register_commands(&mut self, commands: Vec<Command>) {
        if self.intents & GatewayIntent::MESSAGE_CONTENT == 0 {
            log::error!("MESSAGE_CONTENT intent is required for message commands to work");
        }

        commands.into_iter().for_each(|mut command| {
            // if a custom prefix is not applied, add the default prefix
            if !command.custom_prefix {
                command.name = format!(
                    "{default_prefix}{name}",
                    default_prefix = self.prefix,
                    name = command.name
                );
            }

            self.commands.insert(command.name.clone(), command.clone());
        });
    }

    pub async fn register_slash_commands(&mut self, commands: Vec<SlashCommand>) {
        self.slash_commands.extend(
            utils::slash::register_slash_commands(commands)
                .await
                .into_iter(),
        );
    }
}
