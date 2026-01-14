pub mod echo;
pub mod gcp_clear_dynamic_reject;

use std::{collections::HashMap, sync::Arc};

use tonic::async_trait;

use crate::event::{echo::EchoHandler, gcp_clear_dynamic_reject::GcpClearDynamicRejectHandler};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    Echo,
    GCPUATClearDynamicReject,
    GCPTestDbConnection,
    Unknown(String),
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        match s {
            "aws-dgl-echo" => EventType::Echo,
            "aws-gcp-uat-clear-dynamic-reject" => EventType::GCPUATClearDynamicReject,
            _ => EventType::Unknown(s.to_string()),
        }
    }
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, payload: String) -> Result<String, String>;
}

pub struct EventRegistry {
    handlers: HashMap<EventType, Arc<dyn EventHandler>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        let mut registry = EventRegistry {
            handlers: HashMap::new(),
        };
        registry.register(EventType::Echo, Arc::new(EchoHandler));
        registry.register(
            EventType::GCPUATClearDynamicReject,
            Arc::new(GcpClearDynamicRejectHandler),
        );
        registry
    }

    pub fn register(&mut self, event_type: EventType, handler: Arc<dyn EventHandler>) {
        self.handlers.insert(event_type, handler);
    }

    pub async fn dispatch(
        &self,
        event: &str,
        message: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let event_type = EventType::from(event);

        match self.handlers.get(&event_type) {
            Some(handler) => {
                let result = handler.handle(message.to_string()).await?;
                Ok(result)
            }
            None => Err(format!("Unknown event type: {}", event).into()),
        }
    }
}
