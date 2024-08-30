use std::borrow::Cow;

use reqwest::{header::CONTENT_TYPE, Body};
use tauri::{
    http::{HeaderMap, HeaderValue},
    AppHandle,
};
use tauri_plugin_dialog::DialogExt;
use tokio::fs::File;
use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};
use tokio_util::codec::{BytesCodec, FramedRead};
use ws::init::init_ws_connection;
use ws::messages::send_ws_message;

pub mod ws;

pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            join_server,
            leave_server,
            upload_file,
            send_ws_string
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn join_server(handle: AppHandle) {
    init_ws_connection(handle).await;
    // From here WS_CONNECTION is set
    send_ws_message(Message::Text("join".to_string())).await;
    // After this client receives joined message and updates the client count
}

#[tauri::command]
async fn leave_server(handle: AppHandle) {
    send_ws_message(Message::Close(Some(CloseFrame {
        code: CloseCode::Normal,
        reason: Cow::Borrowed("User disconnecting"),
    })))
    .await;
}

#[tauri::command]
async fn send_ws_string(message: String) {
    send_ws_message(Message::Text(message)).await;
}

#[tauri::command]
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
