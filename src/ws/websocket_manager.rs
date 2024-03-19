use log::*;
use nanoserde::DeJson;

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
use crate::consts::{self, payloads};
use crate::handlers::events::Event;
use crate::handlers::EventHandler;
use crate::ws::payload::Payload;
use crate::{models::*, Client};

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
        event_handler: Arc<impl EventHandler + std::marker::Sync + 'static>,
        commands: Arc<HashMap<String, crate::Command>>,
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

            match payload.operation_code {
                OpCode::Dispatch => {
                    let event_handler = Arc::clone(&event_handler);

                    info!(
                        "received {} event\npayload:{}",
                        payload
                            .type_name
                            .as_ref()
                            .map(|i| i.as_str())
                            .unwrap_or("Unknown"),
                        json::parse(&payload.raw_json).unwrap().pretty(4)
                    );

                    let commands = Arc::clone(&commands);
                    tokio::spawn(async move {
                        Self::dispatch_event(payload, event_handler, commands).await;
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
        event_handler: Arc<impl EventHandler>,
        commands: Arc<HashMap<String, crate::Command>>,
    ) {
        let event = Event::from_str(payload.type_name.as_ref().unwrap().as_str()).unwrap();
        match event {
            Event::Ready => {
                let ready_data = ready_response::ReadyResponse::deserialize_json(&payload.raw_json)
                    .expect("Failed to parse json");

                event_handler.ready(ready_data.data).await;

                // const READY_SEQ: usize = 1;
                // if payload.sequence == Some(READY_SEQ) {
                // *self.session_id.lock().unwrap() = Some(ready_data.data.session_id.clone());
                // *self.resume_gateway_url.lock().unwrap() = Some(format!(
                //     "{}/?v=10&encoding=json",
                //     ready_data.data.resume_gateway_url
                // ));
                // }
            }

            Event::MessageCreate => {
                let message_data =
                    message_response::MessageResponse::deserialize_json(&payload.raw_json)
                        .expect("Failed to parse json");

                if let Some(command_name) = message_data.data.content.split(' ').next() {
                    if let Some(handler_fn) = commands.get(command_name) {
                        let handler = handler_fn.clone();
                        handler.call(message_data.data).await;

                        return;
                    }
                }

                event_handler.message_create(message_data.data).await;
            }

            Event::MessageUpdate => {
                let message_data =
                    message_response::MessageResponse::deserialize_json(&payload.raw_json)
                        .expect("Failed to parse json");

                event_handler.message_update(message_data.data).await;
            }

            Event::MessageDelete => {
                let delete_data =
                    deleted_message_response::DeletedMessageResponse::deserialize_json(
                        &payload.raw_json,
                    )
                    .expect("Failed to parse json");

                event_handler.message_delete(delete_data.data).await;
            }

            _ => error!("{event:?} event is not implemented"),
        }
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
