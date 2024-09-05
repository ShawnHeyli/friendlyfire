use std::str::FromStr;

use futures_util::SinkExt;
use log::{debug, error, info};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Url};
use tokio_tungstenite::tungstenite::Message;

use crate::play::{image::ImagePayload, video::VideoPayload};

use super::{WebSocketError, WS_CONNECTION};

enum WsMessage {
    UpdateClientCount(u32),
    PlayImage(Url, String),
    PlayVideo(Url, String),
}

impl FromStr for WsMessage {
    type Err = WebSocketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(';').collect();

        match parts[0] {
            "update_client_count" => {
                let count = parts[1]
                    .parse::<u32>()
                    .map_err(|_| WebSocketError::ParseError("Invalid client count"))?;
                Ok(WsMessage::UpdateClientCount(count))
            }
            "play_image" => {
                let path = parts[1]
                    .parse::<String>()
                    .map_err(|_| WebSocketError::ParseError("Invalid play_image remote path"))?;
                let text = parts[2]
                    .parse::<String>()
                    .map_err(|_| WebSocketError::ParseError("Invalid play_image text"))?;
                Ok(WsMessage::PlayImage(
                    Url::parse(&path).unwrap(),
                    text.to_owned(),
                ))
            }
            "play_video" => {
                let path = parts[1]
                    .parse::<String>()
                    .map_err(|_| WebSocketError::ParseError("Invalid play_video remote path"))?;
                let text = parts[2]
                    .parse::<String>()
                    .map_err(|_| WebSocketError::ParseError("Invalid play_video text"))?;
                Ok(WsMessage::PlayVideo(
                    Url::parse(&path).unwrap(),
                    text.to_owned(),
                ))
            }
            _ => Err(WebSocketError::ParseError("Unknown message type")),
        }
    }
}

pub async fn handle_message(message: Message, handle: AppHandle) {
    debug!("Received {:?} from server", message);
    if let Message::Text(msg) = message {
        match WsMessage::from_str(&msg) {
            Ok(WsMessage::UpdateClientCount(count)) => {
                info!("Updated client count");
                if let Err(e) = handle.emit(
                    "updateClientCount",
                    ClientCount {
                        client_count: count,
                    },
                ) {
                    log::error!("Failed to emit updateClientCount event: {:?}", e);
                }
            }
            Ok(WsMessage::PlayImage(path, text)) => ImagePayload::new(path, text).emit(&handle),
            Ok(WsMessage::PlayVideo(path, text)) => VideoPayload::new(path, text).emit(&handle),
            Err(e) => error!("Failed to parse a WebSocket message: {:?}", e),
        }
    }
}

pub async fn send_ws_message(message: Message) -> Result<(), WebSocketError> {
    if let Some(ws) = WS_CONNECTION.lock().await.as_mut() {
        debug!("Sent '{:?}' to the server", &message);
        ws.send(message).await.map_err(WebSocketError::SendError)?;
    }
    Ok(())
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientCount {
    client_count: u32,
}
