use axum::{
    body::Bytes,
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, DefaultBodyLimit, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use std::{net::SocketAddr, sync::Arc};
use tokio::{fs::File, io::AsyncWriteExt, sync::broadcast};

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState::default());

    let routes = Router::new()
        .route("/", get(|| async { Html::from("Video Sync Server") }))
        .route("/ws", get(ws_handler).with_state(app_state))
        .route("/upload", post(upload).layer(DefaultBodyLimit::disable()))
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}

struct AppState {
    sender: broadcast::Sender<String>,
}

impl Default for AppState {
    fn default() -> Self {
        let (sender, _receiver) = broadcast::channel(32);
        AppState { sender }
    }
}

#[debug_handler]
async fn upload(ConnectInfo(addr): ConnectInfo<SocketAddr>, body: Bytes) -> impl IntoResponse {
    println!("{} accessed /upload", addr);
    let mut file = File::create("stuff").await.unwrap();
    file.write_all(&body).await.unwrap();

    (StatusCode::OK, "File uploaded successfully")
}

#[debug_handler]
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl axum::response::IntoResponse {
    println!("{} accessed /ws", addr);
    ws.on_upgrade(move |socket| handle_socket(socket, app_state, addr))
}

async fn handle_socket(mut socket: WebSocket, app_state: Arc<AppState>, _addr: SocketAddr) {
    let mut rx = app_state.sender.subscribe();
    let tx = app_state.sender.clone();

    loop {
        tokio::select! {
            msg = socket.recv() => {
                if let Some(Ok(msg)) = msg {
                    match msg{
                        Message::Text(msg) => {
                            println!("Received socket: {}", msg);
                            tx.send(msg.clone()).unwrap();
                            println!("Sent channel: {}", msg);
                        },
                        Message::Binary(msg) =>
                            println!("Received socket: {:?}", msg),
                        Message::Ping(msg) =>
                            println!("Received socket: {:?}", msg),
                        Message::Pong(msg) =>
                            println!("Received socket: {:?}", msg),
                        Message::Close(msg) =>
                            println!("Received socket: {:?}", msg),
                    }
                }
            },

            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    println!("Received channel: {}", msg);
                    socket.send(Message::Text(msg.clone())).await.unwrap();
                    println!("Sent socket: {}", msg);
                }
            }
        }
    }
}
