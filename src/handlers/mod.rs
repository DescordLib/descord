use std::future::Future;

use crate::ws::payload::Payload;
use crate::Client;

use crate::models::{
    deleted_message_response::DeletedMessageData, message_response::MessageData,
    ready_response::ReadyData,
};

pub mod events;
