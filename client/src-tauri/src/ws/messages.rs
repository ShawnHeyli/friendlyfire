use futures_util::SinkExt;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio_tungstenite::tungstenite::Message;

use super::WS_CONNECTION;

pub async fn handle_message(message: Message, handle: &AppHandle) {
    match message {
        Message::Text(msg) => match msg.as_str() {
            "update_client_count" => update_client_count(8, handle).await,
            _ => unreachable!(),
        },
        Message::Binary(_) => todo!(),
        Message::Ping(_) => todo!(),
        Message::Pong(_) => todo!(),
        Message::Close(_) => todo!(),
        Message::Frame(_) => unreachable!(),
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientCount {
    client_count: u32,
}

async fn update_client_count(client_count: u32, handle: &AppHandle) {
    handle
        .emit("updateClientCount", ClientCount { client_count })
        .unwrap();
}

pub async fn send_ws_message(message: Message) {
    if let Some(ws) = WS_CONNECTION.lock().await.as_mut() {
        ws.send(message).await.unwrap();
    }
}
