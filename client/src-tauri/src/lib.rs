use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::info;
use reqwest::{header::CONTENT_TYPE, Body};
use serde::{Deserialize, Serialize};
use tauri::{
    http::{HeaderMap, HeaderValue},
    AppHandle, Url,
};
use tauri_plugin_dialog::DialogExt;
use tokio::{fs::File, net::TcpStream};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tokio_util::codec::{BytesCodec, FramedRead};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![join_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WebSocketSplitSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WebSocketSplitStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

lazy_static::lazy_static! {
    static ref WS_CONNECTION: Arc<Mutex<Option<WebSocketSplitSink>>> = Arc::new(Mutex::new(None));
}

#[tauri::command]
async fn join_server() {
    init_ws_connection().await;
    // From here WS_CONNECTION is set
    send_ws_message(Message::Text("joined".to_string())).await;
    // After this client receives joined message and updates the client count
}

async fn send_ws_message(message: Message) {
    if let Some(ws) = WS_CONNECTION.lock().await.as_mut() {
        ws.send(message).await.unwrap();
    }
}

async fn init_ws_connection() {
    let (ws, _) = connect_async("ws://localhost:3000/ws")
        .await
        .inspect(|(_, _)| info!("Successfully connected to the server"))
        .unwrap();
    let (write, read): (WebSocketSplitSink, WebSocketSplitStream) = ws.split();

    let mut ws_connection = WS_CONNECTION.lock().await;
    *ws_connection = Some(write);

    init_keep_alive();
    init_ws_listener(read);
}

fn init_keep_alive() {
    let ws_connection = Arc::clone(&WS_CONNECTION);
    tauri::async_runtime::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(20));

        loop {
            interval.tick().await;
            // send the message and continue if sending is successful.
            if let Some(write) = ws_connection.lock().await.as_mut() {
                if write.send(Message::Ping(vec![1, 3, 3, 7])).await.is_err() {
                    break;
                }
            }
        }
    });
}

fn init_ws_listener(mut read: WebSocketSplitStream) {
    tauri::async_runtime::spawn(async move {
        while let Some(message) = read.next().await {
            println!("{:?}", &message.unwrap());
        }
    });
}

#[derive(Serialize, Deserialize)]
struct PlayMessage {
    media: PlayMessageMedia,
}

#[derive(Serialize, Deserialize)]
struct PlayMessageMedia {
    url: Url,
}

async fn upload_file(handle: AppHandle) {
    let file = handle
        .dialog()
        .file()
        .add_filter("Images *.jpg *.jpeg", &["jpg", "jpeg"])
        .blocking_pick_file();
    if let Some(file) = file {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        if let Some(mime_type) = file.mime_type {
            headers.insert(CONTENT_TYPE, HeaderValue::from_str(&mime_type).unwrap());
        }
        client
            .post("http://localhost:3000/upload")
            .headers(headers)
            .body({
                let stream =
                    FramedRead::new(File::open(file.path).await.unwrap(), BytesCodec::new());
                Body::wrap_stream(stream)
            })
            .send()
            .await
            .unwrap();
    }
}
