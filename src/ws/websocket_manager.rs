// std
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::{clone, thread};

use guild::{GuildCreate, GuildCreateResponse};
use log::*;
use nanoserde::{DeJson, SerJson};
use reqwest::{Method, Url};

use crate::client::BOT_ID;
use crate::client::RESUME_GATEWAY_URL;
use crate::client::SESSION_ID;
use crate::client::TOKEN;
use crate::{internals::*, utils};

// models
use crate::models::interaction::{
    Interaction, InteractionAutoCompleteChoice, InteractionAutoCompleteChoicePlaceholder,
    InteractionAutoCompleteChoices, InteractionResponsePayload,
};
use crate::models::ready_response::ReadyResponse;
use crate::models::*;

use deleted_message_response::DeletedMessageResponse;
use message_response::MessageResponse;
use misc::Reconnect;
use reaction_response::ReactionResponse;
use role_response::*;

// Tokio & Future
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use tokio_tungstenite::tungstenite::{Message, Result};
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async, WebSocketStream};

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{future, pin_mut, SinkExt, StreamExt};

use crate::consts::opcode::OpCode;
use crate::consts::{self, payloads, InteractionCallbackType, InteractionType};
use crate::handlers::events::Event;
use crate::utils::{fetch_channel, fetch_guild, fetch_member, request};
use crate::ws::payload::Payload;
use crate::Client;

use crate::cache::{MESSAGE_CACHE, ROLE_CACHE};
use crate::consts::permissions::ADMINISTRATOR;
use crate::prelude::{Channel, Guild};

type SocketWrite = Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>;
type SocketRead = Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>;

pub struct WsManager {
    token: String,
    socket: (SocketWrite, SocketRead),
    sequence: Arc<Mutex<usize>>,
}

impl WsManager {
    pub async fn new(token: &str) -> Result<Self> {
        Ok(Self {
            token: token.to_owned(),
            socket: {
                info!("Connecting to web socket");
                Self::connect_socket()
                    .await
                    .expect("Failed to connect to the gateway")
            },
            sequence: Arc::new(Mutex::new(0)),
        })
    }

    async fn connect_socket() -> Result<(SocketWrite, SocketRead)> {
        info!("...");

        let (socket, _response) = connect_async(consts::GATEWAY_URL).await?;

        let (write, read) = socket.split();
        let (write, read) = (Arc::new(Mutex::new(write)), Arc::new(Mutex::new(read)));

        info!("connected!");

        Ok((write, read))
    }

    pub async fn start(&mut self, intents: u32, handlers: Handlers) {
        let mut interval = 0;
        loop {
            let e = self.connect(intents, handlers.clone()).await;
            info!("{e:?}");

            if let Ok(false) = e {
                info!("shutting down the bot");
                break;
            }

            error!("Connection closed");
            interval += 1;
            info!("Attempting to reconnect in {} seconds", interval);
            thread::sleep(Duration::from_secs(interval));
            self.socket = Self::reconnect(Arc::clone(&self.sequence)).await;
        }
    }

    // if retuns Ok(false), then it shouldn't try to reconnect
    async fn connect(&mut self, intents: u32, handlers: Handlers) -> Result<bool> {
        if let Some(Ok(Message::Text(body))) = self.socket.1.lock().await.next().await {
            let Some(payload) = Payload::parse(&body) else {
                panic!("Failed to parse json, body: {body}");
            };

            match payload.operation_code {
                OpCode::Hello => {
                    let time_ms = payload.data["heartbeat_interval"].as_u64().unwrap();
                    let writer = Arc::clone(&self.socket.0);
                    let reader = Arc::clone(&self.socket.1);

                    info!("heartbeat interval: {}ms", time_ms);

                    tokio::spawn(async move {
                        Self::heartbeat_start(Duration::from_millis(time_ms), writer, reader).await;
                    });

                    info!("performing handshake");
                    self.identify(intents).await?;
                }

                _ => panic!("Unknown event received when attempting to handshake"),
            }
        }

        loop {
            let x = self.socket.1.lock().await.next().await;
            if let Some(Ok(Message::Text(body))) = x {
                let Some(payload) = Payload::parse(&body) else {
                    error!("Failed to parse json");
                    continue;
                };

                info!("Opcode: {:?}", payload.operation_code);
                if let OpCode::Dispatch = payload.operation_code {
                    let current_seq = payload.sequence.unwrap_or(0);
                    *self.sequence.lock().await = current_seq;
                    info!(
                        "Received {} event, sequence: {current_seq}",
                        payload.type_name.as_deref().unwrap_or("Unknown"),
                        // For Debugging
                        // json::parse(&payload.raw_json).unwrap().pretty(4)
                    );

                    let seq = Arc::clone(&self.sequence);
                    let handlers = handlers.clone();

                    if matches!(
                        Event::from_str(payload.type_name.as_ref().unwrap().as_str()),
                        Ok(Event::Reconnect)
                    ) {
                        self.socket = Self::reconnect(Arc::clone(&self.sequence)).await;
                    }

                    tokio::spawn(async move {
                        Self::dispatch_event(payload, seq, handlers)
                            .await
                            .expect("Failed to parse json response");
                    });
                }
            } else {
                match x.unwrap().unwrap() {
                    Message::Close(Some(cf)) => {
                        self.socket = Self::reconnect(Arc::clone(&self.sequence)).await;
                    }

                    x => error!("Closing the connection, got the error: {x:?}",),
                }

                return Ok(false);
            }
        }

        info!("Exiting...");

        Ok(true)
    }

    async fn dispatch_event(
        payload: Payload,
        seq: Arc<Mutex<usize>>,
        handlers: Handlers,
    ) -> Result<(), nanoserde::DeJsonErr> {
        // info!(
        //     "payload: {}",
        //     json::parse(&payload.raw_json).unwrap().pretty(4)
        // );

        let mut event = match Event::from_str(payload.type_name.as_ref().unwrap().as_str()) {
            Ok(event) => event,
            Err(_) => {
                error!("Failed to parse event from payload type name");
                return Ok(());
            }
        };

        let data = match event {
            Event::Ready => {
                let data = ReadyResponse::deserialize_json(&payload.raw_json).unwrap();

                *RESUME_GATEWAY_URL.lock().unwrap() = Some(data.data.resume_gateway_url.clone());
                *SESSION_ID.lock().unwrap() = Some(data.data.session_id.clone());
                *BOT_ID.lock().unwrap() = Some(data.data.user.id.clone());

                data.data.into()
            }

            Event::MessageCreate => {
                let message_data = MessageResponse::deserialize_json(&payload.raw_json).unwrap();

                MESSAGE_CACHE
                    .lock()
                    .await
                    .put(message_data.data.id.clone(), message_data.data.clone());

                if let Some(command_name) = message_data.data.content.split(' ').next() {
                    if let Some(command_handler_fn) = handlers.commands.get(command_name) {
                        let mut required_permissions: u64 = 0;

                        for permission in &command_handler_fn.permissions {
                            required_permissions |= consts::permissions::parse(permission)
                                .expect("Invalid permission name");
                        }

                        let msg_id = message_data.data.id.clone();
                        let channel_id = message_data.data.channel_id.clone();

                        if required_permissions != 0 {
                            let channel = fetch_channel(&channel_id).await.unwrap();
                            let guild = fetch_guild(channel.guild_id.as_ref().unwrap())
                                .await
                                .unwrap();
                            let data = message_data.data.clone();
                            let user_permissions: u64 = Self::fetch_permissions(
                                data.member.unwrap().roles,
                                data.author.unwrap().id,
                                &guild,
                                Some(&channel),
                            )
                            .await;

                            if user_permissions & required_permissions != required_permissions {
                                utils::send(
                                    &channel_id,
                                    Some(
                                    &msg_id),
                                    "You are missing the required permissions for running this command",
                                )
                                    .await;

                                return Ok(());
                            }
                        }

                        let handler = command_handler_fn.clone();
                        if let Err(e) = command_handler_fn.call(message_data.data.clone()).await {
                            utils::send(&channel_id, Some(&msg_id), e.to_string()).await;
                        }

                        return Ok(());
                    }
                }

                message_data.data.into()
            }

            Event::MessageUpdate => {
                let message_data = MessageResponse::deserialize_json(&payload.raw_json).unwrap();

                MESSAGE_CACHE
                    .lock()
                    .await
                    .put(message_data.data.id.clone(), message_data.data.clone());

                message_data.data.into()
            }

            Event::MessageDelete => {
                let data = DeletedMessageResponse::deserialize_json(&payload.raw_json).unwrap();

                if let Some(cached_data) = MESSAGE_CACHE.lock().await.pop(&data.data.message_id) {
                    if let Some(handler) = handlers
                        .event_handlers
                        .get(&Event::MessageDeleteRaw)
                        .cloned()
                    {
                        let msg_id = data.data.message_id.clone();
                        let channel_id = data.data.channel_id.clone();

                        tokio::spawn(async move {
                            if let Err(e) = handler.call(data.data.into()).await {
                                utils::send(&channel_id, Some(&msg_id), e.to_string()).await;
                            }
                        });
                    }

                    cached_data.into()
                } else {
                    event = Event::MessageDeleteRaw;
                    data.data.into()
                }
            }

            Event::Reconnect => {
                unreachable!("Reconnecting is already handled before this function call")
            }

            Event::GuildRoleCreate => {
                let data = RoleCreateResponse::deserialize_json(&payload.raw_json).unwrap();
                ROLE_CACHE
                    .lock()
                    .await
                    .put(data.data.role.id.clone(), data.data.role.clone());
                data.data.into()
            }

            Event::GuildRoleUpdate => {
                let data = RoleUpdateResponse::deserialize_json(&payload.raw_json).unwrap();
                ROLE_CACHE
                    .lock()
                    .await
                    .put(data.data.role.id.clone(), data.data.role.clone());
                data.data.into()
            }

            Event::GuildRoleDelete => {
                let data = RoleDeleteResponse::deserialize_json(&payload.raw_json).unwrap();
                ROLE_CACHE.lock().await.pop(&data.data.role_id);
                data.data.into()
            }

            Event::MessageReactionAdd => {
                let data = ReactionResponse::deserialize_json(&payload.raw_json).unwrap();
                data.data.into()
            }

            Event::GuildCreate => {
                let data =
                    GuildCreateResponse::deserialize_json(&payload.raw_json).unwrap_or_else(|e| {
                        panic!(
                            "Failing part: {}",
                            &payload.raw_json[e.col - 10..e.col + 10]
                        );
                    });
                data.data.into()
            }

            Event::InteractionCreate => {
                // A band-aid solution
                let mut json = json::parse(&payload.raw_json).unwrap();
                let json =
                    if let json::JsonValue::Array(options) = &mut json["d"]["data"]["options"] {
                        for option in options {
                            option["value"] = json::JsonValue::String(option["value"].to_string());
                        }

                        json.dump()
                    } else {
                        payload.raw_json.clone()
                    };

                let data = InteractionResponsePayload::deserialize_json(&json).unwrap();

                if data.data.type_ == InteractionType::ApplicationCommand as u32 {
                    if let Some(d) = &data.data.data {
                        if let Some(command) = handlers.slash_commands.get(&d.clone().id.unwrap()) {
                            let handler = command.clone();
                            if let Err(e) = handler.call(data.data.clone()).await {
                                data.data.reply(e.to_string(), true).await;
                            };
                        }
                    }
                } else if data.data.type_ == InteractionType::MessageComponent as u32 {
                    let id: &str = data.data.data.as_ref().unwrap().custom_id.as_ref().unwrap();
                    if let Some(component_handler) = handlers.component_handlers.get(id) {
                        component_handler.call(data.data.clone()).await;
                    }
                } else if data.data.type_ == InteractionType::ApplicationCommandAutocomplete as u32
                {
                    let slash_command = handlers
                        .slash_commands
                        .get(data.data.data.as_ref().unwrap().id.as_ref().unwrap())
                        .unwrap();
                    let options = &data.data.data.as_ref().unwrap().options.as_ref().unwrap();

                    for (idx, itm) in options.iter().enumerate() {
                        if itm.focused.unwrap_or(false) {
                            // SAFETY: We are sure that the fn_param_autocomplete is not None
                            let choices = slash_command.fn_param_autocomplete[idx].unwrap()(
                                itm.value.clone(),
                            )
                            .await
                            .into_iter()
                            .map(|i| InteractionAutoCompleteChoice {
                                name: i.clone(),
                                value: i,
                            })
                            .collect();

                            request(
                                Method::POST,
                                &format!(
                                    "/interactions/{}/{}/callback",
                                    data.data.id, data.data.token
                                ),
                                Some(
                                    json::parse(
                                        &InteractionAutoCompleteChoices::new(choices)
                                            .serialize_json(),
                                    )
                                    .unwrap(),
                                ),
                            )
                            .await;
                        }
                    }
                }

                data.data.into()
            }

            _ => {
                info!("{event:?} event is not implemented");
                return Ok(());
            }
        };

        if let Some(handler) = handlers.event_handlers.get(&event) {
            // TODO: pass context data along with the error for error reporting
            handler.call(data).await;
        }

        Ok(())
    }

    async fn reconnect(seq: Arc<Mutex<usize>>) -> (SocketWrite, SocketRead) {
        info!("Reopening the connection...");

        let resume_gateway_url = RESUME_GATEWAY_URL.lock().unwrap().as_ref().unwrap().clone();
        let token = TOKEN.lock().unwrap().as_ref().unwrap().clone();
        let session_id = SESSION_ID.lock().unwrap().as_ref().unwrap().clone();
        let seq = *seq.lock().await;

        let (mut socket, _) = connect_async(Url::parse(&resume_gateway_url).unwrap().as_str())
            .await
            .unwrap();

        socket
            .send(Message::Text(json::stringify(payloads::resume(
                &token,
                &session_id,
                seq,
            ))))
            .await
            .expect("Failed to send resume event");

        let (write, read) = socket.split();
        let (write, read) = (Arc::new(Mutex::new(write)), Arc::new(Mutex::new(read)));
        (write, read)
    }

    async fn heartbeat_start(
        heartbeat_interval: Duration,
        writer: SocketWrite,
        reader: SocketRead,
    ) {
        let mut last_sequence: usize = 0;
        loop {
            let message = Message::Text(json::stringify(payloads::heartbeat(last_sequence)));
            info!("sending heartbeat");
            writer
                .lock()
                .await
                .send(message)
                .await
                .expect("Failed to send heartbeat");

            // TODO: if it fails, reconnect

            tokio::time::sleep(heartbeat_interval).await;
            last_sequence += 1;
        }
    }

    async fn identify(&self, intents: u32) -> Result<()> {
        self.send_text(json::stringify(payloads::identify(&self.token, intents)))
            .await
    }

    async fn send_text(&self, msg: String) -> Result<()> {
        self.socket.0.lock().await.send(Message::Text(msg)).await
    }

    async fn fetch_permissions(
        roles: Vec<String>,
        id: String,
        guild: &Guild,
        channel: Option<&Channel>,
    ) -> u64 {
        // Check if member is the guild owner
        if guild.owner_id == id {
            return ADMINISTRATOR;
        }

        // Start with default role permissions or bot-specific permissions
        let mut base_permissions = guild
            .default_role()
            .await
            .unwrap()
            .permissions
            .parse::<u64>()
            .unwrap();

        // Aggregate permissions from member's roles
        for role_id in &roles {
            if let Ok(role) = guild.fetch_role(role_id).await {
                base_permissions |= role.permissions.parse::<u64>().unwrap();
            }
        }

        // Administrator check
        if base_permissions & ADMINISTRATOR == ADMINISTRATOR {
            return ADMINISTRATOR;
        }

        // Apply permission overwrites if channel is provided
        if let Some(channel) = channel {
            if let Some(overwrites) = &channel.permission_overwrites {
                for overwrite in overwrites {
                    let allow = overwrite.allow.parse::<u64>().unwrap();
                    let deny = overwrite.deny.parse::<u64>().unwrap();

                    if overwrite.overwrite_type == 1 && overwrite.id == id {
                        // Member specific overwrites
                        base_permissions &= !deny;
                        base_permissions |= allow;
                    } else if overwrite.overwrite_type == 0 && roles.contains(&overwrite.id) {
                        // Role specific overwrites
                        base_permissions &= !deny;
                        base_permissions |= allow;
                    }
                }
            }
        }

        base_permissions
    }
}

pub struct Handlers {
    pub event_handlers: Arc<HashMap<Event, EventHandler>>,
    pub commands: Arc<HashMap<String, Command>>,
    pub slash_commands: Arc<HashMap<String, SlashCommand>>,
    pub component_handlers: Arc<HashMap<String, ComponentHandler>>,
}

impl Clone for Handlers {
    fn clone(&self) -> Self {
        Handlers {
            event_handlers: Arc::clone(&self.event_handlers),
            commands: Arc::clone(&self.commands),
            slash_commands: Arc::clone(&self.slash_commands),
            component_handlers: Arc::clone(&self.component_handlers),
        }
    }
}
