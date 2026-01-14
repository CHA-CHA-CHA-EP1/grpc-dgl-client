pub mod aws_delete_mock_face_scan_8_sec;
pub mod aws_mock_face_scan_8_sec;
pub mod echo;
pub mod gcp_clear_dynamic_reject;
pub mod gcp_delete_loan_appid;

use std::{collections::HashMap, sync::Arc};

use tonic::async_trait;

use crate::event::{
    aws_delete_mock_face_scan_8_sec::AwsDeleteMockFaceScan8Sec,
    aws_mock_face_scan_8_sec::AwsMockFaceScan8Sec, echo::EchoHandler,
    gcp_clear_dynamic_reject::GcpClearDynamicRejectHandler,
    gcp_delete_loan_appid::GcpDeleteLoanAppIdHandler,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    Echo,
    GCPUATClearDynamicReject,
    GCPUATDeleteLoanAppId,
    GCPTestDbConnection,
    AWSMockFaceScan8Sec,
    AWSDeleteMockFaceScan8Sec,
    Unknown(String),
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        match s {
            "aws-dgl-echo" => EventType::Echo,
            "aws-gcp-uat-clear-dynamic-reject" => EventType::GCPUATClearDynamicReject,
            "aws-gcp-uat-delete-loan-appid" => EventType::GCPUATDeleteLoanAppId,
            "aws-mock-face-scan-8-sec" => EventType::AWSMockFaceScan8Sec,
            "aws-delete-mock-face-scan-8-sec" => EventType::AWSDeleteMockFaceScan8Sec,
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
        registry.register(
            EventType::GCPUATDeleteLoanAppId,
            Arc::new(GcpDeleteLoanAppIdHandler),
        );
        registry.register(
            EventType::AWSMockFaceScan8Sec,
            Arc::new(AwsMockFaceScan8Sec),
        );
        registry.register(
            EventType::AWSDeleteMockFaceScan8Sec,
            Arc::new(AwsDeleteMockFaceScan8Sec),
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
