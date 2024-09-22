use std::{
    borrow::Cow,
    path::PathBuf,
    time::{self, Duration},
};

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::info;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tokio::{
    fs::File,
    net::TcpStream,
    sync::{broadcast, Mutex},
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
    MaybeTlsStream, WebSocketStream,
};
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Default)]
struct ServerState {
    url: Option<tauri::Url>,
}

impl ServerState {
    fn upload_url(&self) -> Option<tauri::Url> {
        self.url.as_ref().map(|url| {
            let mut url = url.clone();
            url.set_scheme("https").unwrap();
            url.set_path("upload");
            url
        })
    }

    fn ws_url(&self) -> Option<tauri::Url> {
        self.url.as_ref().map(|url| {
            let mut url = url.clone();
            url.set_scheme("wss").unwrap();
            url.set_path("ws");
            url
        })
    }

    fn remote_media(&self, remote_path: String) -> Option<tauri::Url> {
        self.url.as_ref().map(|url| {
            let mut url = url.clone();
            url.set_scheme("https").unwrap();
            url.set_path(format!("uploads/{}", remote_path).as_str());
            url
        })
    }
}

#[derive(Default)]
struct ConnectionState {
    ws_connection: Option<WebSocketSplitSink>,
    kill_channel: Option<broadcast::Sender<()>>,
}

pub type WebSocketSplitSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

#[derive(Serialize, Deserialize)]
struct MediaMessage {
    media_url: tauri::Url,
    top_message: String,
    bottom_message: String,
    sender: User,
    timeout: time::Duration,
}

impl From<MediaMessage> for Message {
    fn from(val: MediaMessage) -> Self {
        Message::Text(serde_json::to_string(&val).unwrap())
    }
}

#[derive(Serialize, Deserialize, Default)]
struct User {
    // avatar_url: Option<tauri::Url>,
    username: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            connect_to_server,
            disconnect_from_server,
            send_media
        ])
        .setup(|app| {
            app.manage(Mutex::new(ServerState::default()));
            app.manage(Mutex::new(ConnectionState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn connect_to_server(handle: AppHandle, domain: String) -> Result<(), String> {
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
        tauri::async_runtime::spawn(async move {
            loop {
                tokio::select! {
                    msg = read.next() => {
                        println!("{:?}", msg);
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

#[tauri::command]
async fn disconnect_from_server(handle: AppHandle) {
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

#[tauri::command]
async fn send_media(
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
            let message = MediaMessage {
                media_url: remote_path,
                top_message,
                bottom_message,
                sender: user,
                timeout: time::Duration::from_secs(timeout),
            };
            ws.send(message.into()).await.unwrap();
        }
    }
}
