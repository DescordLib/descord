use crate::ws::payload::Payload;

pub mod events;

pub trait EventHandler {
    fn ready(&self, payload: Payload) {}
    fn message_create(&self, payload: Payload) {}
}
