use std::{borrow::Cow, fmt};

use futures_util::SinkExt;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

use crate::ConnectionState;

use super::ServerState;

#[derive(Debug)]
pub enum DisconnectError {
    WebSocket(tokio_tungstenite::tungstenite::Error),
    KillChannel(tokio::sync::broadcast::error::SendError<()>),
}

impl fmt::Display for DisconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisconnectError::WebSocket(err) => write!(f, "WebSocket send error: {}", err),
            DisconnectError::KillChannel(err) => {
                write!(f, "Kill channel send error: {}", err)
            }
        }
    }
}

pub async fn disconnect(handle: AppHandle) -> Result<(), DisconnectError> {
    let mutex_state = handle.state::<Mutex<ConnectionState>>();
    let mut state = mutex_state.lock().await;

    // Close WebSocket connection
    if let Some(ws) = &mut state.ws_connection {
        ws.send(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::Borrowed("User disconnecting"),
        })))
        .await
        .map_err(DisconnectError::WebSocket)?;
        state.ws_connection = None;
    }

    // Send kill signal
    if let Some(kill_channel) = &mut state.kill_channel {
        kill_channel
            .send(())
            .map_err(DisconnectError::KillChannel)?;
        state.kill_channel = None;
    }

    // Clear server state
    let mutex_state = handle.state::<Mutex<ServerState>>();
    let mut state = mutex_state.lock().await;
    state.set_url(None);

    Ok(())
}
