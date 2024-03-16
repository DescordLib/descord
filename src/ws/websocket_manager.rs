use log::*;

use std::net::TcpStream;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::Message;
use tungstenite::Result;
use tungstenite::WebSocket;
use url::Url;

use crate::consts::opcode::OpCode;
use crate::consts::{self, payloads};
use crate::handlers::events::Event;
use crate::handlers::EventHandler;
use crate::ws::payload::Payload;

type Ws = Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>;

#[derive(Clone)]
pub struct WsManager {
    socket: Ws,
    resume_gateway_url: Arc<Mutex<Option<String>>>,
    session_id: Arc<Mutex<Option<String>>>,
    last_sequence: Arc<Mutex<Option<usize>>>,
}

impl WsManager {
    pub fn new() -> Result<Self> {
        let (socket, _response) = connect(Url::parse(consts::GATEWAY_URL).unwrap())?;

        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
            resume_gateway_url: Arc::new(Mutex::new(None)),
            session_id: Arc::new(Mutex::new(None)),
            last_sequence: Arc::new(Mutex::new(None)),
        })
    }

    pub fn connect(&self, token: &str, intents: u32, event_handler: impl EventHandler) -> Result<()> {
        loop {
            let body = if let Ok(data) = self.socket.lock().unwrap().read() {
                data.into_text().unwrap()
            } else {
                continue;
            };

            let Some(payload) = Payload::parse(&body) else {
                error!("Failed to parse json, body: {body}");
                continue;
            };

            *self.last_sequence.lock().unwrap() = payload.sequence;

            match payload.operation_code {
                OpCode::Hello => {
                    info!("starting heartheat");
                    self.heartbeat_start(
                        Duration::from_millis(payload.data["heartbeat_interval"].as_u64().unwrap()),
                        token.to_string(),
                    );

                    info!("performing handshake");
                    self.identify(token, intents)?;
                }

                OpCode::Dispatch => {
                    info!("event {} received", payload.type_name.as_ref().unwrap());

                    const READY_SEQ: usize = 1;
                    if payload.sequence == Some(READY_SEQ) {
                        *self.session_id.lock().unwrap() =
                            Some(payload.data["session_id"].as_str().unwrap().to_string());

                        *self.resume_gateway_url.lock().unwrap() = Some(format!(
                            "{}/?v=10&encoding=json",
                            payload.data["resume_gateway_url"].as_str().unwrap()
                        ));
                    }

                    let event = Event::from_str(payload.type_name.as_ref().unwrap().as_str()).unwrap();
                    event.handle(&event_handler, payload);
                }

                _ => {}
            }
        }
    }

    fn heartbeat_start(&self, heartbeat_interval: Duration, token: String) {
        let socket = Arc::clone(&self.socket);
        let resume_gateway_url = Arc::clone(&self.resume_gateway_url);
        let session_id = Arc::clone(&self.session_id);
        let last_sequence = Arc::clone(&self.last_sequence);

        thread::Builder::new()
            .name("heartbeat thread".to_string())
            .spawn(move || loop {
                info!("sending heartbeat");
                if let Err(tungstenite::Error::AlreadyClosed) =
                    socket.lock().unwrap().send(Message::Text(json::stringify(
                        payloads::heartbeat(last_sequence.lock().unwrap().unwrap_or(0)),
                    )))
                {
                    warn!("connection closed");
                    info!("Reopening the connection...");
                    let (mut socket, _response) = connect(
                        Url::parse(
                            resume_gateway_url
                                .lock()
                                .unwrap()
                                .as_ref()
                                .unwrap()
                                .as_str(),
                        )
                        .unwrap(),
                    )
                    .unwrap();

                    socket
                        .send(Message::Text(json::stringify(payloads::resume(
                            token.as_str(),
                            session_id.lock().unwrap().as_ref().unwrap().as_str(),
                            last_sequence.lock().unwrap().unwrap(),
                        ))))
                        .unwrap();
                }

                thread::sleep(heartbeat_interval);
            })
            .unwrap();
    }

    fn identify(&self, token: &str, intents: u32) -> Result<()> {
        self.send_text(json::stringify(payloads::identify(token,intents)))
    }

    fn send_text(&self, msg: String) -> Result<()> {
        self.socket.lock().unwrap().send(Message::Text(msg))
    }
}
