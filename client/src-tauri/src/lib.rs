use std::collections::HashMap;

use futures_util::{SinkExt, StreamExt};
use log::info;
use reqwest::{header::CONTENT_TYPE, Body};
use tauri::{
    http::{HeaderMap, HeaderValue},
    AppHandle,
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
                .level(log::LevelFilter::Trace)
                .build(),
        )
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![send_ws_message, upload_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn connect_ws() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, ()> {
    let (ws, _) = connect_async("ws://localhost:3000/ws")
        .await
        .inspect(|(_, _)| info!("Successfully connected to the server"))
        .unwrap();
    Ok(ws)
}

#[tauri::command]
async fn send_ws_message(message: String) {
    let (mut write, _) = connect_ws().await.unwrap().split();
    write.send(Message::Text(message)).await.unwrap()
}

#[tauri::command]
fn upload_file(handle: AppHandle) {
    handle
        .dialog()
        .file()
        .add_filter("Images *.jpeg *.jpg", &["jpg", "jpeg"])
        .pick_file(|file| {
            if let Some(file) = file {
                tauri::async_runtime::spawn(async {
                    let client = reqwest::Client::new();
                    let mut headers = HeaderMap::new();
                    if let Some(mime_type) = file.mime_type {
                        headers.insert(CONTENT_TYPE, HeaderValue::from_str(&mime_type).unwrap());
                    }
                    client
                        .post("http://localhost:3000/upload")
                        .headers(headers)
                        .body({
                            let stream = FramedRead::new(
                                File::open(file.path).await.unwrap(),
                                BytesCodec::new(),
                            );
                            Body::wrap_stream(stream)
                        })
                        .send()
                        .await
                        .unwrap();
                });
            }
        })
}
