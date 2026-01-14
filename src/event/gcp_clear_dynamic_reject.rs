use tonic::async_trait;

use crate::{event::EventHandler, helper::gcp_db_connection::gcp_create_db_pool};

pub struct GcpClearDynamicRejectHandler;

#[async_trait]
impl EventHandler for GcpClearDynamicRejectHandler {
    async fn handle(&self, payload: String) -> Result<String, String> {
        let cif_num = payload;
        Ok("Ok".to_string())
    }
}
