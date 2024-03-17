use std::future::Future;

use crate::client::Context;
use crate::ws::payload::Payload;
use crate::Client;

use crate::models::{message_response::MessageData, ready_response::ReadyData};

pub mod events;

use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: std::marker::Send + std::marker::Sync {
    async fn ready(&self, ctx: &Context, ready_data: ReadyData) {}
    async fn message_create(&self, ctx: &Context, message_data: MessageData) {}
}
