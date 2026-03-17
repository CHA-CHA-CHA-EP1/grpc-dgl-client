use tonic::async_trait;

use crate::event::EventHandler;

pub struct AwsMockAPiError;

#[derive(Debug)]
pub struct Request {}

#[async_trait]
impl EventHandler for AwsMockAPiError {
    async fn handle(&self, payload: String) -> Result<String, String> {
        todo!()
    }
}
