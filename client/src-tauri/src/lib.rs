use play::{pick_image, upload_file};
use tauri::AppHandle;
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
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            join_server,
            leave_server,
            send_ws_string,
            play_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn join_server(handle: AppHandle) {
    init_ws_connection(handle).await;
    // From here WS_CONNECTION is set
    // After this client receives joined message and updates the client count
}

#[tauri::command]
async fn leave_server() {
    close_ws_connection().await;
}

#[tauri::command]
async fn send_ws_string(message: String) {
    send_ws_message(Message::Text(message)).await;
}

#[tauri::command]
async fn play_image(handle: AppHandle, text: String) {
    if let Some(file) = pick_image(&handle) {
        let filename = upload_file(file).await;
        send_ws_message(Message::Text(format!("play_image;{};{}", filename, text))).await
    }
}
