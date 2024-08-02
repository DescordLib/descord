use lazy_static::lazy_static;
use reqwest::Method;
use std::collections::HashMap;

use std::sync::Mutex;

use json::object;
use nanoserde::SerJson;

use crate::consts::intents::GatewayIntent;
use crate::internals::*;
use crate::models::application_command::ApplicationCommandOption;
use crate::prelude::{CreateMessageData, Embed, Message};
use crate::utils::{self, request};
use crate::ws;
use crate::{consts, internals, Event};

use log::{error, info};

// SAFETY: These will always be valid if accessed from an event.
lazy_static::lazy_static! {
    pub(crate) static ref BOT_ID: Mutex<Option<String>> = Mutex::new(None);
    pub(crate) static ref SESSION_ID: Mutex<Option<String>> = Mutex::new(None);
    pub(crate) static ref TOKEN: Mutex<Option<String>> = Mutex::new(None);
    pub(crate) static ref RESUME_GATEWAY_URL: Mutex<Option<String>> = Mutex::new(None);

    static ref HELP_EMBED: tokio::sync::Mutex<Embed> = tokio::sync::Mutex::new(Embed::default());
}

pub struct Client {
    intents: u32,
    ws: ws::WsManager,
    token: String,
    commands: HashMap<String, Command>,
    slash_commands: HashMap<String, SlashCommand>,
    event_handlers: HashMap<Event, EventHandler>,
    component_handlers: HashMap<String, ComponentHandler>,
    prefix: String,
}

impl Client {
    pub async fn new(token: &str, intents: impl Into<u32>, prefix: &str) -> Self {
        *TOKEN.lock().unwrap() = Some(token.to_owned());

        Self {
            intents: intents.into(),
            token: token.to_owned(),
            ws: ws::WsManager::new(token)
                .await
                .expect("Failed to initialize websockets"),
            prefix: prefix.to_owned(),

            commands: HashMap::new(),
            slash_commands: HashMap::new(),
            event_handlers: HashMap::new(),
            component_handlers: HashMap::new(),
        }
    }

    pub async fn login(mut self) {
        self.default_help().await;
        self.ws
            .start(
                self.intents,
                ws::Handlers {
                    event_handlers: self.event_handlers.into(),
                    commands: self.commands.into(),
                    slash_commands: self.slash_commands.into(),
                    component_handlers: self.component_handlers.into(),
                },
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

    pub fn register_component_callbacks(&mut self, commands: Vec<ComponentHandler>) {
        self.component_handlers
            .extend(commands.into_iter().map(|d| (d.id.clone(), d)));
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

    /// Returns info about all registered message commands.
    /// Might be useful for creating a help command.
    pub fn get_commands(&self) -> Vec<CommandInfo> {
        if self.commands.is_empty() {
            log::warn!("No commands are registered make sure to call `enable_default_help` or `get_commands` after registering them.");
        }

        self.commands
            .iter()
            .map(|(_, value)| CommandInfo {
                name: value.name.clone(),
                description: value.description.clone(),
                params: value.fn_sig.clone(),
            })
            .collect()
    }

    /// Returns info about all registered slash commands.
    /// Might be useful for creating a help command.
    pub fn get_slash_commands(&self) -> Vec<SlashCommandInfo> {
        if self.commands.is_empty() {
            log::warn!("No slash commands are registered make sure to call `enable_default_help` or `get_slash_commands` after registering them.");
        }

        self.slash_commands
            .iter()
            .map(|(_, value)| SlashCommandInfo {
                name: value.name.clone(),
                description: value.description.clone(),
                params: value
                    .fn_param_names
                    .iter()
                    .cloned()
                    .zip(value.fn_sig.iter().cloned())
                    .collect(),
            })
            .collect()
    }

    /// Adds a default help command that lists all registered commands.
    async fn default_help(&mut self) {
        let help_cmd = format!("{}help", self.prefix);
        if self.commands.iter().any(|(name, _)| name == &help_cmd) {
            return;
        }

        let mut commands_field_text = format!("`{}help` - Sends this help message", self.prefix);
        let mut slash_commands_field_text = String::new();

        self.get_commands().into_iter().for_each(|command| {
            commands_field_text += &format!("\n`{}` - {}", command.name, command.description);
        });

        self.get_slash_commands().into_iter().for_each(|command| {
            slash_commands_field_text +=
                &format!("`/{}` - {}\n", command.name, command.description);
        });

        let help_embed = crate::prelude::EmbedBuilder::new()
            .color(crate::color::Color::Green)
            .title("Help has arrived!")
            .field("Message Commands", &commands_field_text, false)
            .field("Slash Commands", &slash_commands_field_text, false)
            .build();

        *HELP_EMBED.lock().await = help_embed;

        fn f(
            msg: Message,
            _: Vec<internals::Value>,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = crate::DescordResult> + Send + 'static>,
        > {
            Box::pin(async move {
                msg.reply(HELP_EMBED.lock().await.clone()).await;
                Ok(())
            })
        }

        self.commands.insert(
            format!("{}help", self.prefix),
            Command {
                name: "help".to_string(),
                custom_prefix: false,
                fn_sig: vec![],
                handler_fn: f,
                optional_params: vec![],
                permissions: vec![],
                description: "Sends this help message".to_string(),
            },
        );
    }
}

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub params: Vec<ParamType>,
}

#[derive(Debug, Clone)]
pub struct SlashCommandInfo {
    pub name: String,
    pub description: String,
    pub params: Vec<(String, ParamType)>,
}
