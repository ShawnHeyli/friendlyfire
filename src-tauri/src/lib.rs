mod media;
mod message;
mod server;

use std::path::PathBuf;

use futures_util::stream::{SplitSink, SplitStream};
use server::ServerState;
use tauri::{AppHandle, Manager};
use tokio::{
    net::TcpStream,
    sync::{broadcast, Mutex},
};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub type WebSocketSplitSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub type WebSocketSplitStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[derive(Default)]
struct ConnectionState {
    ws_connection: Option<WebSocketSplitSink>,
    kill_channel: Option<broadcast::Sender<()>>,
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
            let player = app.get_webview_window("player").unwrap();
            player.set_decorations(false)?;
            player.set_closable(false)?;
            player.set_minimizable(false)?;
            player.set_visible_on_all_workspaces(true)?;
            player.set_always_on_top(true)?;
            player.set_resizable(false)?;
            player.set_skip_taskbar(true)?;
            player.hide()?;

            app.manage(Mutex::new(ServerState::default()));
            app.manage(Mutex::new(ConnectionState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn connect_to_server(handle: AppHandle, domain: String) -> Result<(), String> {
    server::connect(handle, domain)
        .await
        .map_err(|error| format!("{}", error))
}

#[tauri::command]
async fn disconnect_from_server(handle: AppHandle) -> Result<(), String> {
    server::disconnect(handle)
        .await
        .map_err(|error| format!("{}", error))
}

#[tauri::command]
async fn send_media(
    handle: AppHandle,
    filepath: PathBuf,
    top_message: String,
    bottom_message: String,
    user: media::User,
    timeout: u64,
) -> Result<(), String> {
    media::send(handle, filepath, top_message, bottom_message, user, timeout)
        .await
        .map_err(|error| format!("{}", error))
}
