use crate::consts::intents::GatewayIntent;
use crate::handlers::EventHandler;
use crate::ws::WsManager;

pub struct Client {
    intents: u32,
    ws: WsManager,
    token: String,
}

impl Client {
    pub fn new(token: &str, intents: impl Into<u32>) -> Self {
        Self {
            intents: intents.into(),
            token: token.to_owned(),
            ws: WsManager::new(token).expect("Failed to initialize websockets"),
        }
    }

    pub fn login(&mut self, event_handler: impl EventHandler) {
        self.ws.connect(self.intents, event_handler);
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
