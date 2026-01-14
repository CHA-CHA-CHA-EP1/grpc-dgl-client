use serde_json::json;
use tonic::async_trait;

use crate::event::EventHandler;

pub struct AwsMockFaceScan8Sec;

#[async_trait]
impl EventHandler for AwsMockFaceScan8Sec {
    async fn handle(&self, payload: String) -> Result<String, String> {
        let url = "http://orch-application-form-mgmt.dgl-application.svc.cluster.local:8080/scenario/set-mock-response";

        let body = json!({
            "cdiToken": payload,
            "path": "/api/v1/application/auth-biometrics/result",
            "mockResponse": {
                "httpCode": 400,
                "body": {
                    "code": "DGL0027",
                    "message": "pending face scan result"
                }
            }
        });

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        if status.is_success() {
            Ok(format!("Success: {}", response_text))
        } else {
            Err(format!(
                "Request failed with status {}: {}",
                status, response_text
            ))
        }
    }
}
