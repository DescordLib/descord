use super::*;

implemented_enum! {
    pub enum HandlerValue {
        ReadyData,
        Message,
        DeletedMessage,
        Reaction,
        GuildCreate,
        Interaction,
        RoleDelete,
        RoleEvent,
        Reconnect,
    }
}

#[derive(Debug, Clone)]
pub struct EventHandler {
    pub event: Event,
    pub handler_fn: EventHandlerFn,
}

pub type EventHandlerFn = fn(
    HandlerValue,
) -> std::pin::Pin<
    Box<dyn futures_util::Future<Output = DescordResult> + Send + 'static>,
>;

impl EventHandler {
    pub async fn call(&self, data: HandlerValue) -> DescordResult {
        let fut = ((self.handler_fn)(data));
        let boxed_fut: std::pin::Pin<
            Box<dyn std::future::Future<Output = DescordResult> + Send + 'static>,
        > = Box::pin(fut);
        boxed_fut.await
    }
}
