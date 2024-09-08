use log::debug;
use play::image::{self, pick_image, ImagePayload};
use play::video::{self, pick_video, VideoPayload};
use play::{upload_file, Sendable};
use tauri::{AppHandle, Url};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tokio_tungstenite::tungstenite::Message;
use ws::close::close_ws_connection;
use ws::init::init_ws_connection;
use ws::messages::send_ws_message;

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
async fn join_server(handle: AppHandle) {
    init_ws_connection(handle).await.unwrap();
    // From here WS_CONNECTION is set
    // After this client receives joined message and updates the client count
}

#[tauri::command]
async fn leave_server() {
    close_ws_connection().await.unwrap();
}

#[tauri::command]
async fn send_ws_string(message: String) {
    send_ws_message(Message::Text(message)).await.unwrap();
}

#[tauri::command]
async fn play_image(handle: AppHandle, text: String) {
    if let Some(file) = pick_image(&handle) {
        let (width, height) = image::dimensions(&file.path).unwrap();
        let remote_path = upload_file(file).await;
        debug!("Received remote_path '{}' from the server", remote_path);
        let remote_path =
            Url::parse(format!("https://localhost:7331/uploads/{}", remote_path).as_str()).unwrap();
        let payload = ImagePayload::new(remote_path, text.clone(), width, height);
        payload.send().await;
    }
}

#[tauri::command]
async fn play_video(handle: AppHandle, text: String) {
    if let Some(file) = pick_video(&handle) {
        let (width, height) = video::dimensions(&file.path).unwrap();
        let remote_path = upload_file(file).await;
        debug!("Received remote_path '{}' from the server", remote_path);
        let remote_path =
            Url::parse(format!("https://localhost:7331/uploads/{}", remote_path).as_str()).unwrap();
        let payload = VideoPayload::new(remote_path, text.clone(), width, height);
        payload.send().await;
    }
}
