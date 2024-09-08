use std::{sync::Arc, time::Duration};

use futures_util::{SinkExt, StreamExt};
use log::info;
use tauri::AppHandle;
use tokio::time;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::{
    messages::handle_message, WebSocketError, WebSocketSplitSink, WebSocketSplitStream,
    WS_CONNECTION,
};

pub async fn init_ws_connection(handle: AppHandle) -> Result<(), WebSocketError> {
    if WS_CONNECTION.lock().await.is_none() {
        let (ws, _) = connect_async("wss://localhost:7331/ws")
            .await
            .inspect(|(_, _)| info!("Successfully connected to the server"))
            .map_err(WebSocketError::ConnectionError)?;
        let (write, read): (WebSocketSplitSink, WebSocketSplitStream) = ws.split();

        let mut ws_connection = WS_CONNECTION.lock().await;
        *ws_connection = Some(write);
        drop(ws_connection);

        init_keep_alive();
        init_ws_listener(read, handle);
    }
    Ok(())
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

fn init_ws_listener(mut read: WebSocketSplitStream, handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        while let Some(Ok(message)) = read.next().await {
            handle_message(message, handle.clone()).await;
        }
    });
}
