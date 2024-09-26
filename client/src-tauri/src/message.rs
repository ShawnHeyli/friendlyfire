use std::fmt::Display;

use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio_tungstenite::tungstenite::Message;

use crate::{media::MediaMessage, server::ClientCountMessage, ConnectionState};

#[derive(Serialize, Deserialize, Debug)]
pub enum WsMessage {
    Media(MediaMessage),
    ClientCount(ClientCountMessage),
}

#[derive(Debug)]
pub enum WsError {
    SerializationError(serde_json::Error),
    SendError(tokio_tungstenite::tungstenite::Error),
}

impl Display for WsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WsError::SerializationError(error) => {
                write!(f, "Could not serialize WebSocket message: {}", error)
            }
            WsError::SendError(error) => write!(f, "Unable to send message: {}", error),
        }
    }
}

impl WsMessage {
    pub async fn send(self, handle: &AppHandle) -> Result<(), WsError> {
        let mutex_state = handle.state::<tokio::sync::Mutex<ConnectionState>>();
        let mut state = mutex_state.lock().await;
        if let Some(ws) = &mut state.ws_connection {
            let message = serde_json::to_string(&self).map_err(WsError::SerializationError)?;
            ws.send(Message::text(message))
                .await
                .map_err(WsError::SendError)?;
        }
        Ok(())
    }
}
