use axum::{
    body::{Body, Bytes},
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, DefaultBodyLimit, Path, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
    sync::broadcast::{self, Sender},
    time::sleep,
};
use tokio_util::io::ReaderStream;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState::default());

    let routes = Router::new()
        .route("/", get(|| async { Html::from("Video Sync Server") }))
        .route("/ws", get(ws_handler).with_state(app_state.clone()))
        .route("/healthcheck", get(healthcheck))
        .route("/upload", post(upload).layer(DefaultBodyLimit::disable()))
        .route("/uploads/:asset", get(serve_asset))
        .into_make_service_with_connect_info::<SocketAddr>();

    match tokio::net::TcpListener::bind("0.0.0.0:7331").await {
        Ok(listener) => {
            if let Err(e) = axum::serve(listener, routes).await {
                eprintln!("Server error: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to bind to address: {:?}", e);
        }
    }
}

struct AppState {
    sender: broadcast::Sender<Message>,
}

impl Default for AppState {
    fn default() -> Self {
        let (sender, _receiver) = broadcast::channel(32);
        AppState { sender }
    }
}

async fn healthcheck(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    println!("{} accessed /healthcheck", addr);
    (StatusCode::OK, "Ok")
}

#[debug_handler]
async fn upload(ConnectInfo(addr): ConnectInfo<SocketAddr>, body: Bytes) -> impl IntoResponse {
    println!("{} accessed /upload", addr);
    let filename = Alphanumeric.sample_string(&mut rand::thread_rng(), 24);
    match File::create(format!("uploads/{}", &filename)).await {
        Ok(mut file) => {
            if let Err(e) = file.write_all(&body).await {
                eprintln!("Failed to write file: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to write file".to_string(),
                );
            }
            // Delete the file after 2min
            let filename_clone = filename.clone();
            tokio::spawn(async move {
                sleep(Duration::from_secs(120)).await;
                if let Err(e) = fs::remove_file(format!("uploads/{}", &filename_clone)).await {
                    eprintln!("Unable to delete file: {:?}", e);
                } else {
                    println!("File deleted: {}", &filename_clone);
                }
            });

            (StatusCode::OK, filename)
        }
        Err(e) => {
            eprintln!("Failed to create file: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create file".to_string(),
            )
        }
    }
}

async fn serve_asset(
    Path(path): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("{} accessed /uploads/{}", addr, path);
    let path = format!("uploads/{}", path);
    match File::open(path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);
            (StatusCode::OK, body)
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Body::from("File not found".to_string()),
        ),
    }
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

    handle_client_connect(tx.clone());

    loop {
        tokio::select! {
            msg= socket.recv() => {
                if let Some(Ok(msg)) = msg{
                    match msg{
                        Message::Text(msg) => {
                            println!("Received socket: {:?}", &msg);
                            if let Err(e) = tx.send(Message::Text(msg)) {
                                eprintln!("Failed to send message: {:?}", e);
                            }
                        },
                        Message::Binary(msg) =>
                            println!("Received socket: {:?}", &msg),
                        Message::Ping(msg) =>
                            println!("Received socket: {:?}", &msg),
                        Message::Pong(msg) =>
                            println!("Received socket: {:?}", &msg),
                        Message::Close(msg) => {
                            println!("Received socket: {:?}", &msg);
                            handle_client_disconnect(tx.clone());
                            drop(rx);
                            drop(tx);
                            break;
                        }
                    }
                } else {
                    handle_client_disconnect(tx.clone());
                }
            },

            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    println!("Received channel: {:?}", &msg);
                    if let Err(e) = socket.send(msg.clone()).await {
                        println!("Failed to send message: {:?}", e);
                        handle_client_disconnect(tx.clone());
                        break;
                    }
                    println!("Sent socket: {:?}", &msg);
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum WsMessage {
    ClientCount(ClientCountMessage),
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientCountMessage {
    client_count: u64,
}

fn handle_client_connect(tx: Sender<Message>) {
    let client_count = tx.receiver_count() as u64;
    let message = WsMessage::ClientCount(ClientCountMessage { client_count });
    if let Err(e) = tx.send(Message::Text(serde_json::to_string(&message).unwrap())) {
        eprintln!("Failed to send client count update: {:?}", e);
    }
}

fn handle_client_disconnect(tx: Sender<Message>) {
    let client_count = (tx.receiver_count() - 1) as u64;
    let message = WsMessage::ClientCount(ClientCountMessage { client_count });
    if let Err(e) = tx.send(Message::Text(serde_json::to_string(&message).unwrap())) {
        eprintln!("Failed to send client count update: {:?}", e);
    }
}
