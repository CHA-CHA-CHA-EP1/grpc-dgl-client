use tonic::async_trait;

use crate::event::EventHandler;

pub struct EchoHandler;

#[async_trait]
impl EventHandler for EchoHandler {
    async fn handle(&self, payload: String) -> Result<String, String> {
        Ok("Ok".to_string())
    }
}
