use std::{borrow::Cow, time::Duration};

use futures_util::{SinkExt, StreamExt};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_http::reqwest;
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
};

use crate::{media::MediaMessage, ConnectionState};

#[derive(Default)]
pub struct ServerState {
    url: Option<tauri::Url>,
}

impl ServerState {
    pub fn upload_url(&self) -> Option<tauri::Url> {
        self.url.as_ref().map(|url| {
            let mut url = url.clone();
            url.set_scheme("https").unwrap();
            url.set_path("upload");
            url
        })
    }

    pub fn ws_url(&self) -> Option<tauri::Url> {
        self.url.as_ref().map(|url| {
            let mut url = url.clone();
            url.set_scheme("wss").unwrap();
            url.set_path("ws");
            url
        })
    }

    pub fn remote_media(&self, remote_path: String) -> Option<tauri::Url> {
        self.url.as_ref().map(|url| {
            let mut url = url.clone();
            url.set_scheme("https").unwrap();
            url.set_path(format!("uploads/{}", remote_path).as_str());
            url
        })
    }
}

pub async fn connect(handle: AppHandle, domain: String) -> Result<(), String> {
    let mutex_state = handle.state::<Mutex<ServerState>>();
    let mut state = mutex_state.lock().await;
    state.url = Some(reqwest::Url::parse(format!("https://{}", domain).as_str()).unwrap());
    let ws_url = state.ws_url().unwrap();

    let mutex_state = handle.state::<Mutex<ConnectionState>>();
    let mut state = mutex_state.lock().await;

    if state.ws_connection.is_none() {
        let (ws, _) = connect_async(ws_url.to_string())
            .await
            .inspect(|(_, _)| info!("Successfully connected to the server"))
            .unwrap();
        let (write, mut read) = ws.split();
        state.ws_connection = Some(write);

        // kill channel
        let (sender, _) = broadcast::channel::<()>(8);
        state.kill_channel = Some(sender.clone());

        // Spawn a task to handle incoming messages from the server
        let mut listener_kill = sender.subscribe();
        let handle_clone = handle.clone();
        tauri::async_runtime::spawn(async move {
            loop {
                tokio::select! {
                    msg = read.next() => {
                        match msg {
                            Some(Ok(message)) => handle_message(&handle_clone, message),
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

        // Spawn a task to handle incoming messages from the server
        let mut keepalive_kill = sender.subscribe();
        let handle_clone = handle.clone();
        tauri::async_runtime::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                tokio::select! {
                _ = interval.tick() => {
                    let mutex_state = handle_clone.state::<Mutex<ConnectionState>>();
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
        Ok(())
    } else {
        Err("Connection has already been set".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WsMessage {
    Media(MediaMessage),
    ClientCount(ClientCountMessage),
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

pub async fn disconnect(handle: AppHandle) {
    let mutex_state = handle.state::<Mutex<ConnectionState>>();
    let mut state = mutex_state.lock().await;

    if let Some(ws) = &mut state.ws_connection {
        ws.send(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::Borrowed("User disconnecting"),
        })))
        .await
        .unwrap();
        state.ws_connection = None;
    }

    if let Some(kill_channel) = &mut state.kill_channel {
        kill_channel.send(()).unwrap();
        state.kill_channel = None;
    }

    let mutex_state = handle.state::<Mutex<ServerState>>();
    let mut state = mutex_state.lock().await;
    state.url = None;
}
