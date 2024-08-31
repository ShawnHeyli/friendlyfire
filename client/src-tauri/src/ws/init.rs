use std::{borrow::Cow, sync::Arc, time::Duration};

use futures_util::{SinkExt, StreamExt};
use log::info;
use tauri::AppHandle;
use tokio::time;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
};

use super::{
    messages::{handle_message, send_ws_message},
    WebSocketSplitSink, WebSocketSplitStream, WS_CONNECTION,
};

pub async fn init_ws_connection(handle: AppHandle) {
    if WS_CONNECTION.lock().await.is_none() {
        let (ws, _) = connect_async("ws://localhost:3000/ws")
            .await
            .inspect(|(_, _)| info!("Successfully connected to the server"))
            .unwrap();
        let (write, read): (WebSocketSplitSink, WebSocketSplitStream) = ws.split();

        let mut ws_connection = WS_CONNECTION.lock().await;
        *ws_connection = Some(write);
        drop(ws_connection);

        init_keep_alive();
        init_ws_listener(read, handle);
    }
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

pub async fn close_ws_connection() {
    if WS_CONNECTION.lock().await.is_some() {
        send_ws_message(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::Borrowed("User disconnecting"),
        })))
        .await;

        let mut ws_connection = WS_CONNECTION.lock().await;
        *ws_connection = None;
    }
}
