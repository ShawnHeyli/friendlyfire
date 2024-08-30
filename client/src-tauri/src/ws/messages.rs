use std::str::FromStr;

use futures_util::SinkExt;
use log::info;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio_tungstenite::tungstenite::Message;

use super::WS_CONNECTION;

enum WsMessage {
    UpdateClientCount(u32),
}

trait MessageExtractor {
    type Output;
    fn extract_value(&self) -> Self::Output;
}

impl MessageExtractor for WsMessage {
    type Output = Result<u32, String>;

    fn extract_value(&self) -> Self::Output {
        match self {
            WsMessage::UpdateClientCount(count) => Ok(*count),
        }
    }
}

impl FromStr for WsMessage {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(';').collect();
        if parts.len() != 2 {
            return Err("Invalid message format");
        }

        match parts[0] {
            "update_client_count" => {
                let count = parts[1]
                    .parse::<u32>()
                    .map_err(|_| "Invalid client count")?;
                Ok(WsMessage::UpdateClientCount(count))
            }
            _ => Err("Unknown message type"),
        }
    }
}

pub async fn handle_message(message: Message, handle: AppHandle) {
    println!("{:?}", message);
    match message {
        Message::Text(msg) => match WsMessage::from_str(&msg).unwrap() {
            WsMessage::UpdateClientCount(count) => update_client_count(count, handle).await,
        },
        Message::Binary(data) => println!("Received binary data of size {}", data.len()),
        Message::Ping(data) => println!("Received a ping {:?}", data),
        Message::Pong(data) => println!("Received a pong {:?}", data),
        Message::Close(data) => println!("Received a close frame {:?}", data),
        Message::Frame(_) => unreachable!(),
    }
}

pub async fn send_ws_message(message: Message) {
    if let Some(ws) = WS_CONNECTION.lock().await.as_mut() {
        ws.send(message).await.unwrap();
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientCount {
    client_count: u32,
}

async fn update_client_count(client_count: u32, handle: AppHandle) {
    info!("Updated client count");
    handle
        .emit("updateClientCount", ClientCount { client_count })
        .unwrap();
}
