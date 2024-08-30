use futures_util::SinkExt;
use log::info;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio_tungstenite::tungstenite::Message;

use super::WS_CONNECTION;

pub async fn handle_message(message: Message, handle: AppHandle) {
    match message {
        Message::Text(msg) => match msg.as_str() {
            "update_client_count" => update_client_count(8, handle).await,
            _ => unreachable!(),
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
