use std::str::FromStr;

use futures_util::SinkExt;
use log::info;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio_tungstenite::tungstenite::Message;

use crate::play::handle_image;

use super::WS_CONNECTION;

enum WsMessage {
    UpdateClientCount(u32),
    PlayImage(String),
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
            "play_image" => {
                let path = parts[1]
                    .parse::<String>()
                    .map_err(|_| "Problem ocurred while getting image path")
                    .unwrap();
                Ok(WsMessage::PlayImage(path.to_owned()))
            }
            _ => Err("Unknown message type"),
        }
    }
}

pub async fn handle_message(message: Message, handle: AppHandle) {
    println!("{:?}", message);
    match message {
        Message::Text(msg) => match WsMessage::from_str(&msg).unwrap() {
            WsMessage::UpdateClientCount(count) => update_client_count(count, handle),
            WsMessage::PlayImage(path) => handle_image(path, handle),
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

fn update_client_count(client_count: u32, handle: AppHandle) {
    info!("Updated client count");
    handle
        .emit("updateClientCount", ClientCount { client_count })
        .unwrap();
}
