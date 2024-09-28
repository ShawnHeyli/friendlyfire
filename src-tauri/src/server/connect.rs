use futures_util::SinkExt;
use std::{fmt, time::Duration};

use futures_util::StreamExt;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::server::state::ServerState;
use crate::{message::WsMessage, ConnectionState, WebSocketSplitStream};
#[derive(Debug)]
pub enum ConnectError {
    UrlParseError(url::ParseError),
    ConnectionError(String),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectError::UrlParseError(err) => write!(f, "URL parse error: {}", err),
            ConnectError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
        }
    }
}
pub async fn connect(handle: AppHandle, domain: String) -> Result<(), ConnectError> {
    let mutex_state = handle.state::<Mutex<ServerState>>();
    let mut state = mutex_state.lock().await;
    set_server_url(&mut state, domain).await?;
    let ws_url = state
        .ws_url()
        .ok_or_else(|| ConnectError::ConnectionError("WebSocket URL is not set".to_string()))?;

    let mutex_state = handle.state::<Mutex<ConnectionState>>();
    let mut state = mutex_state.lock().await;

    if state.ws_connection.is_none() {
        establish_ws_connection(ws_url.to_string(), &mut state, &handle).await?;
        Ok(())
    } else {
        Err(ConnectError::ConnectionError(
            "Connection has already been set".to_string(),
        ))
    }
}

async fn set_server_url(state: &mut ServerState, domain: String) -> Result<(), ConnectError> {
    let url = tauri::Url::parse(format!("https://{}", domain).as_str())
        .map_err(ConnectError::UrlParseError)?;
    state.set_url(Some(url));
    Ok(())
}

async fn establish_ws_connection(
    ws_url: String,
    state: &mut ConnectionState,
    handle: &AppHandle,
) -> Result<(), ConnectError> {
    let (ws, _) = connect_async(ws_url)
        .await
        .inspect(|(_, _)| info!("Successfully connected to the server"))
        .map_err(|_| ConnectError::ConnectionError("Failed to connect to WebSocket".to_string()))?;

    let (write, read) = ws.split();
    state.ws_connection = Some(write);

    // Channel to kill the tasks on disconnect
    let (sender, _) = broadcast::channel::<()>(8);
    state.kill_channel = Some(sender.clone());

    // Spawn a task to handle incoming messages from the server (and other users)
    spawn_message_listener(sender.clone(), handle.clone(), read);

    // Spawn to send keepalive messages to the server
    // otherwise the connection dies after 20-30 seconds
    spawn_keepalive_task(sender, handle.clone());

    Ok(())
}

fn spawn_message_listener(
    sender: broadcast::Sender<()>,
    handle: AppHandle,
    mut read: WebSocketSplitStream,
) {
    let mut listener_kill = sender.subscribe();
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::select! {
                msg = read.next() => {
                    match msg {
                        Some(Ok(message)) => handle_message(&handle, message),
                        Some(Err(_error)) => {
                            // handle error
                        }
                        None => {
                            // handle end of stream
                        }
                    }
                }
                _ = listener_kill.recv() => {
                    break;
                }
            }
        }
    });
}

fn spawn_keepalive_task(sender: broadcast::Sender<()>, handle: AppHandle) {
    let mut keepalive_kill = sender.subscribe();
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let mutex_state = handle.state::<Mutex<ConnectionState>>();
                    let mut state = mutex_state.lock().await;
                    if let Some(ws_sink) = &mut state.ws_connection {
                        ws_sink.send(Message::Ping(vec![])).await.unwrap();
                    }
                }
                _ = keepalive_kill.recv() => {
                    break;
                }
            }
        }
    });
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientCountMessage {
    client_count: u64,
}

fn handle_message(handle: &AppHandle, message: Message) {
    if let Message::Text(message) = message {
        match serde_json::from_str::<WsMessage>(message.as_str()) {
            Ok(ws_message) => match ws_message {
                WsMessage::Media(message) => {
                    debug!("{:?}", message);
                    handle
                        .emit_to("player", "ff://media_play", message)
                        .unwrap();
                }
                WsMessage::ClientCount(message) => {
                    debug!("{:?}", message);
                    handle
                        .emit_to("player", "ff://client_count", message)
                        .unwrap();
                }
            },
            Err(error) => {
                eprintln!("Error: {:?} cause by {:?}", error, message)
            }
        }
    }
}
