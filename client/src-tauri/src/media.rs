use futures_util::SinkExt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tokio::{fs::File, sync::Mutex};
use tokio_tungstenite::tungstenite::Message;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    server::{ServerState, WsMessage},
    ConnectionState,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaMessage {
    media_url: tauri::Url,
    top_message: String,
    bottom_message: String,
    sender: User,
    timeout: u64, // in milliseconds
}

impl From<MediaMessage> for Message {
    fn from(val: MediaMessage) -> Self {
        Message::Text(serde_json::to_string(&val).unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    // avatar_url: Option<tauri::Url>,
    username: String,
}

pub async fn send(
    handle: AppHandle,
    filepath: PathBuf,
    top_message: String,
    bottom_message: String,
    user: User,
    timeout: u64,
) {
    let file = File::open(filepath).await.unwrap();
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);

    let mutex_state = handle.state::<Mutex<ServerState>>();
    let state = mutex_state.lock().await;

    if let Some(url) = state.upload_url().clone() {
        let client = reqwest::Client::new();
        let res = client.post(url).body(body).send().await.unwrap();
        let remote_path = res.text().await.unwrap();

        // File uploaded on the server from now

        let remote_path = state.remote_media(remote_path).unwrap();
        let mutex_state = handle.state::<Mutex<ConnectionState>>();
        let mut state = mutex_state.lock().await;

        if let Some(ws) = &mut state.ws_connection {
            let message = WsMessage::Media(MediaMessage {
                media_url: remote_path,
                top_message,
                bottom_message,
                sender: user,
                timeout,
            });
            ws.send(Message::text(serde_json::to_string(&message).unwrap()))
                .await
                .unwrap();
        }
    }
}
