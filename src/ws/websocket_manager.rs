use guild::{GuildCreate, GuildCreateResponse};
use log::*;
use nanoserde::{DeJson, SerJson};
use reqwest::Method;

use crate::client::BOT_ID;
use crate::internals::*;

use crate::models::interaction::{
    Interaction, InteractionAutoCompleteChoice, InteractionAutoCompleteChoicePlaceholder,
    InteractionAutoCompleteChoices, InteractionResponsePayload,
};
use crate::models::ready_response::ReadyResponse;
use crate::models::*;
use crate::utils::send_request;
use deleted_message_response::DeletedMessageResponse;
use message_response::MessageResponse;
use reaction_response::ReactionResponse;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::{clone, thread};

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{future, pin_mut, SinkExt, StreamExt};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use tokio_tungstenite::tungstenite::{Message, Result};
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async, WebSocketStream};
use url::Url;

use crate::consts::opcode::OpCode;
use crate::consts::{self, payloads, InteractionCallbackType, InteractionType};
use crate::handlers::events::Event;
use crate::ws::payload::Payload;
use crate::Client;

use crate::cache::MESSAGE_CACHE;

type SocketWrite = Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>;
type SocketRead = Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>;

pub struct WsManager {
    token: String,
    socket: (SocketWrite, SocketRead),
}

impl WsManager {
    pub async fn new(token: &str) -> Result<Self> {
        let (socket, _response) = connect_async(Url::parse(consts::GATEWAY_URL).unwrap()).await?;

        let (write, read) = socket.split();
        let (write, read) = (Arc::new(Mutex::new(write)), Arc::new(Mutex::new(read)));

        Ok(Self {
            token: token.to_owned(),
            socket: (write, read),
        })
    }

    pub async fn connect<'a>(
        &'a self,
        intents: u32,
        event_handlers: Arc<HashMap<Event, EventHandler>>,
        commands: Arc<HashMap<String, Command>>,
        slash_commands: Arc<HashMap<String, SlashCommand>>,
    ) -> Result<()> {
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

        while let Some(Ok(Message::Text(body))) = self.socket.1.lock().await.next().await {
            let Some(payload) = Payload::parse(&body) else {
                error!("Failed to parse json");
                continue;
            };

            info!("Opcode: {:?}", payload.operation_code);
            match payload.operation_code {
                OpCode::Dispatch => {
                    info!(
                        "received {} event",
                        payload
                            .type_name
                            .as_ref()
                            .map(|i| i.as_str())
                            .unwrap_or("Unknown"),
                        // json::parse(&payload.raw_json).unwrap().pretty(4)
                    );

                    let event_handlers = Arc::clone(&event_handlers);
                    let commands = Arc::clone(&commands);
                    let slash_commands = Arc::clone(&slash_commands);

                    tokio::spawn(async move {
                        Self::dispatch_event(payload, event_handlers, commands, slash_commands)
                            .await
                            .expect("Failed to parse json response");
                    });
                }

                _ => {}
            }
        }

        info!("Exiting...");

        Ok(())
    }

    async fn dispatch_event(
        payload: Payload,
        event_handlers: Arc<HashMap<Event, EventHandler>>,
        commands: Arc<HashMap<String, Command>>,
        slash_commands: Arc<HashMap<String, SlashCommand>>,
    ) -> Result<(), nanoserde::DeJsonErr> {
        let mut event = Event::from_str(payload.type_name.as_ref().unwrap().as_str()).unwrap();
        let data = match event {
            Event::Ready => {
                let data = ReadyResponse::deserialize_json(&payload.raw_json).unwrap();

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
                    if let Some(handler_fn) = commands.get(command_name) {
                        let handler = handler_fn.clone();
                        handler.call(message_data.data).await;

                        return Ok(());
                    }
                }

                message_data.data.into()
            }

            Event::MessageUpdate => {
                let message_data = MessageResponse::deserialize_json(&payload.raw_json).unwrap();

                if let Some(cached_message) =
                    MESSAGE_CACHE.lock().await.get_mut(&message_data.data.id)
                {
                    *cached_message = message_data.data.clone();
                }

                message_data.data.into()
            }

            Event::MessageDelete => {
                let data = DeletedMessageResponse::deserialize_json(&payload.raw_json).unwrap();

                if let Some(cached_data) = MESSAGE_CACHE.lock().await.pop(&data.data.message_id) {
                    if let Some(handler) = event_handlers.get(&Event::MessageDeleteRaw).cloned() {
                        tokio::spawn(async move {
                            handler.call(data.data.into()).await;
                        });
                    }

                    cached_data.into()
                } else {
                    event = Event::MessageDeleteRaw;
                    data.data.into()
                }
            }

            Event::MessageReactionAdd => {
                let data = ReactionResponse::deserialize_json(&payload.raw_json).unwrap();
                data.data.into()
            }

            Event::GuildCreate => {
                let data = GuildCreateResponse::deserialize_json(&payload.raw_json).unwrap();
                data.data.into()
            }

            Event::InteractionCreate => {
                let data = InteractionResponsePayload::deserialize_json(&payload.raw_json).unwrap();

                if data.data.type_ == InteractionType::ApplicationCommand as u32 {
                    if let Some(d) = &data.data.data {
                        if let Some(command) = slash_commands.get(&d.clone().id.unwrap()) {
                            let handler = command.clone();
                            handler.call(data.data.clone()).await;
                        }
                    }
                } else if data.data.type_ == InteractionType::ApplicationCommandAutocomplete as u32
                {
                    let slash_command = slash_commands
                        .get(data.data.data.as_ref().unwrap().id.as_ref().unwrap())
                        .unwrap();
                    let options = &data.data.data.as_ref().unwrap().options.as_ref().unwrap();

                    for (idx, itm) in options.iter().enumerate() {
                        if itm.focused.unwrap_or(false) {
                            // SAFETY: this block will only be ran when `fn_param_autocomplete` is some,
                            // so it is safe to unwrap
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

                            send_request(
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

        if let Some(handler) = event_handlers.get(&event) {
            handler.call(data).await;
        }

        Ok(())
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

            tokio::time::sleep(heartbeat_interval).await;
            last_sequence += 1;
        }

        // let socket = Arc::clone(&self.socket);
        // let resume_gateway_url = Arc::clone(&self.resume_gateway_url);
        // let session_id = Arc::clone(&self.session_id);
        // let last_sequence = Arc::clone(&self.last_sequence);
        // let token = self.token.clone();

        // loop {
        // info!("sending heartbeat");
        // if let Err(tungstenite::Error::AlreadyClosed) =
        //     socket
        //         .lock()
        //         .unwrap()
        //         .send(Message::Text(json::stringify(payloads::heartbeat(
        //             last_sequence.lock().unwrap().unwrap_or(0),
        //         ))))
        // {
        //     warn!("connection closed");
        //     info!("Reopening the connection...");
        //     let (mut socket, _response) = connect(
        //         Url::parse(
        //             resume_gateway_url
        //                 .lock()
        //                 .unwrap()
        //                 .as_ref()
        //                 .unwrap()
        //                 .as_str(),
        //         )
        //         .unwrap(),
        //     )
        //     .unwrap();

        //     socket
        //         .send(Message::Text(json::stringify(payloads::resume(
        //             &token,
        //             session_id.lock().unwrap().as_ref().unwrap().as_str(),
        //             last_sequence.lock().unwrap().unwrap(),
        //         ))))
        //         .unwrap();
        // }

        // thread::sleep(heartbeat_interval);
        // }
    }

    async fn identify(&self, intents: u32) -> Result<()> {
        self.send_text(json::stringify(payloads::identify(&self.token, intents)))
            .await
    }

    async fn send_text(&self, msg: String) -> Result<()> {
        self.socket.0.lock().await.send(Message::Text(msg)).await
    }
}
