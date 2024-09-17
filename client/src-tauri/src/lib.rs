use std::path::PathBuf;

use log::{debug, info};
use play::image::{self, ImagePayload};
use play::video::{self, VideoPayload};
use play::{upload_file, Sendable};
use tauri::{AppHandle, Url};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tokio::fs::File;
use tokio_tungstenite::tungstenite::Message;
use ws::close::close_ws_connection;
use ws::init::init_ws_connection;
use ws::messages::send_ws_message;
use ws::WebSocketError;

pub mod play;
pub mod ws;

pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .with_colors(ColoredLevelConfig::default())
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            join_server,
            leave_server,
            send_ws_string,
            play_image,
            play_video,
        ])
        .setup(|_app| {
            rustls::crypto::ring::default_provider()
                .install_default()
                .expect("Failed to install rustls crypto provider");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn join_server(handle: AppHandle) -> Result<(), WebSocketError> {
    init_ws_connection(handle).await
    // From here WS_CONNECTION is set
    // After this client receives joined message and updates the client count
}

#[tauri::command]
async fn leave_server() -> Result<(), WebSocketError> {
    close_ws_connection().await
}

#[tauri::command]
async fn send_ws_string(message: String) {
    send_ws_message(Message::Text(message)).await.unwrap();
}

#[tauri::command]
async fn play_image(path: PathBuf, text: String) {
    info!("{:?}", path);
    let file = File::open(&path).await.unwrap();
    let (width, height) = image::dimensions(&path).unwrap();
    let remote_path = upload_file(file).await;
    debug!("Received remote_path '{}' from the server", remote_path);
    let remote_path =
        Url::parse(format!("http://localhost:7331/uploads/{}", remote_path).as_str()).unwrap();
    let payload = ImagePayload::new(remote_path, text.clone(), width, height);
    payload.send().await;
}

#[tauri::command]
async fn play_video(path: PathBuf, text: String) {
    let file = File::open(&path).await.unwrap();
    let (width, height) = video::dimensions(&path).unwrap();
    let remote_path = upload_file(file).await;
    debug!("Received remote_path '{}' from the server", remote_path);
    let remote_path =
        Url::parse(format!("http://localhost:7331/uploads/{}", remote_path).as_str()).unwrap();
    let payload = VideoPayload::new(remote_path, text.clone(), width, height);
    payload.send().await;
}
