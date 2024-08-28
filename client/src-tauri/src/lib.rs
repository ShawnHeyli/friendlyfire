use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::info;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_websocket::init())
        .invoke_handler(tauri::generate_handler![send_ws_message])
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
