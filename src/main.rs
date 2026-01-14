pub mod tunnel {
    tonic::include_proto!("tunnel.v1");
}

use std::env::{self, args};
use std::sync::Arc;

use grpc_client::event::EventRegistry;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tunnel::tunnel_service_client::TunnelServiceClient;
use tunnel::{AgentMessage, RegisterInfo, ResponseInfo, agent_message};

pub fn parse_message(raw_message: &str) -> (String, String) {
    if let Some((event, message)) = raw_message.split_once(' ') {
        (event.to_string(), message.to_string())
    } else {
        ("unknown".to_string(), raw_message.to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);

    let cluster = args.next().unwrap_or("dgl-aws-sit".to_string());

    let mut client = TunnelServiceClient::connect("http://127.0.0.1:50051").await?;
    let (tx, mut rx) = mpsc::channel::<AgentMessage>(100);

    let registry = Arc::new(EventRegistry::new());

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

    tokio::spawn(async move {
        while let Some(result) = inbound.next().await {
            match result {
                Ok(msg) => {
                    let (event, message) = parse_message(&msg.message);
                    let response = match registry.dispatch(&event, &message).await {
                        Ok(resp) => resp,
                        Err(e) => format!("Error: {}", e),
                    };

                    println!("Response: {}", response);

                    let _ = tx_clone
                        .send(AgentMessage {
                            payload: Some(agent_message::Payload::Response(ResponseInfo {
                                message: response,
                            })),
                        })
                        .await;
                }
                Err(e) => {
                    eprintln!("err: {}", e);
                    break;
                }
            }
        }
        println!("Disconnected");
    });

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        println!("Still alive...");
    }
}
