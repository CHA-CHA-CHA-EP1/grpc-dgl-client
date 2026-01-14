pub mod tunnel {
    tonic::include_proto!("tunnel.v1");
}

use std::env;
use std::sync::Arc;
use std::time::Duration;

use grpc_client::event::EventRegistry;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tonic::transport::Endpoint;
use tunnel::tunnel_service_client::TunnelServiceClient;
use tunnel::{AgentMessage, RegisterInfo, ResponseInfo, agent_message};

pub fn parse_message(raw_message: &str) -> (String, String) {
    if let Some((event, message)) = raw_message.split_once(' ') {
        (event.to_string(), message.to_string())
    } else {
        ("unknown".to_string(), raw_message.to_string())
    }
}

async fn connect_and_run(
    cluster: String,
    registry: Arc<EventRegistry>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to server...");

    let endpoint = Endpoint::from_static("https://cha14.xyz")
        .http2_adaptive_window(true)
        .http2_keep_alive_interval(Duration::from_secs(30))
        .keep_alive_timeout(Duration::from_secs(10))
        .keep_alive_while_idle(true)
        .timeout(Duration::from_secs(60));

    let mut client = TunnelServiceClient::connect(endpoint).await?;
    let (tx, mut rx) = mpsc::channel::<AgentMessage>(100);

    tx.send(AgentMessage {
        payload: Some(agent_message::Payload::Register(RegisterInfo {
            agent_name: cluster,
        })),
    })
    .await?;

    let outbound = async_stream::stream! {
        while let Some(msg) = rx.recv().await {
            yield msg;
        }
    };

    let mut inbound = client.connect_tunnel(outbound).await?.into_inner();
    let tx_clone = tx.clone();

    println!("Connected successfully!");

    while let Some(result) = inbound.next().await {
        match result {
            Ok(msg) => {
                let (event, message) = parse_message(&msg.message);
                let response = match registry.dispatch(&event, &message).await {
                    Ok(resp) => resp,
                    Err(e) => format!("Error: {}", e),
                };

                println!("Response: {}", response);

                if let Err(e) = tx_clone
                    .send(AgentMessage {
                        payload: Some(agent_message::Payload::Response(ResponseInfo {
                            message: response,
                        })),
                    })
                    .await
                {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Stream error: {}", e);
                return Err(e.into());
            }
        }
    }

    println!("Connection closed");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let cluster = args.next().unwrap_or("dgl-aws-sit".to_string());
    let registry = Arc::new(EventRegistry::new());

    loop {
        match connect_and_run(cluster.clone(), registry.clone()).await {
            Ok(_) => {
                println!("Disconnected gracefully");
            }
            Err(e) => {
                eprintln!("Connection error: {}, reconnecting in 5 seconds...", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
