use std::time::Duration;

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::info;
use tauri::{AppHandle, Manager};
use tokio::{
    net::TcpStream,
    sync::{broadcast, Mutex},
};
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
    kill_channel: Option<broadcast::Sender<()>>,
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
    };
    Ok(())
}
