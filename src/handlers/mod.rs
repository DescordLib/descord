use crate::ws::payload::Payload;

use crate::models::{message_response::MessageData, ready_response::ReadyData};

pub mod events;

pub trait EventHandler {
    fn ready(&self, ready_data: ReadyData) {}
    fn message_create(&self, message_data: MessageData) {}
}
