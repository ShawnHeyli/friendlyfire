use futures_util::{stream::SplitSink, StreamExt};
use log::info;
use tauri::{AppHandle, Manager};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

// #[derive(Default)]
// struct ServerState {}

// #[derive(Default)]
// struct PlayerState {}

// #[derive(Default)]
// struct MediaState {}

#[derive(Default)]
struct ConnectionState {
    ws_connection: Option<WebSocketSplitSink>,
}

pub type WebSocketSplitSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![connect_to_server])
        .setup(|app| {
            app.manage(Mutex::new(ConnectionState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn connect_to_server(handle: AppHandle, domain: String) -> Result<(), String> {
    let mutex_state = handle.state::<Mutex<ConnectionState>>();
    let mut state = mutex_state.lock().await;

    if state.ws_connection.is_none() {
        let (ws, _) = connect_async(format!("wss://{}/ws", domain))
            .await
            .inspect(|(_, _)| info!("Successfully connected to the server"))
            .unwrap();
        let (write, mut read) = ws.split();

        state.ws_connection = Some(write);

        // Spawn a task to handle incoming messages from the server
        tokio::spawn(async move {
            while let Some(_msg) = read.next().await {
                // Handle incoming messages here
                todo!();
            }
        });
    };
    Ok(())
}
