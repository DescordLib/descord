use std::future::Future;

use crate::client::Context;
use crate::ws::payload::Payload;
use crate::Client;

use crate::models::{message_response::MessageData, ready_response::ReadyData, deleted_message_response::DeletedMessageData};

pub mod events;

use async_trait::async_trait;

#[async_trait]
pub trait EventHandler: std::marker::Send + std::marker::Sync {
    /// Called when the client becomes ready to start working.
    async fn ready(&self, ctx: &Context, ready_data: ReadyData) {}

    /// Called whenever a message is sent.
    async fn message_create(&self, ctx: &Context, data: MessageData) {}

    /// Called whenever a message is update i.e. content change.
    async fn message_update(&self, ctx: &Context, data: MessageData) {}

    /// Called whenever a message is deleted.
    async fn message_delete(&self, ctx: &Context, data: DeletedMessageData) {}
}
