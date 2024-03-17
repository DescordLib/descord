use std::cell::RefCell;
use std::sync::{Arc, Mutex, MutexGuard};

use json::object;
use nanoserde::SerJson;

use crate::consts;
use crate::consts::intents::GatewayIntent;
use crate::handlers::EventHandler;
use crate::prelude::{CreateMessageData, MessageData};
use crate::ws::WsManager;

pub struct Client {
    intents: u32,
    ws: WsManager,
    token: String,
}

impl Client {
    pub async fn new(token: &str, intents: impl Into<u32>) -> Self {
        Self {
            intents: intents.into(),
            token: token.to_owned(),
            ws: WsManager::new(token)
                .await
                .expect("Failed to initialize websockets"),
        }
    }

    pub async fn login(&self, event_handler: impl EventHandler + std::marker::Sync + 'static) {
        self.ws.connect(self.intents, event_handler.into()).await;
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
