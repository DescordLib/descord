use super::*;

pub type ComponentHandlerFn =
    fn(Interaction) -> std::pin::Pin<Box<dyn futures_util::Future<Output = DescordResult> + Send + 'static>>;


#[derive(Debug, Clone)]
pub struct ComponentHandler {
    pub id: String,
    pub handler_fn: ComponentHandlerFn,
}

impl ComponentHandler {
    pub async fn call(&self, data: Interaction) -> DescordResult {
        let fut = ((self.handler_fn)(data));
        let boxed_fut: std::pin::Pin<
            Box<dyn std::future::Future<Output = DescordResult> + Send + 'static>,
        > = Box::pin(fut);

        boxed_fut.await
    }
}
