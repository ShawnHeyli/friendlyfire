pub mod image;
pub mod video;

use reqwest::Body;
use serde::Serialize;
use tauri::AppHandle;
use tokio::fs::File;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::codec::{BytesCodec, FramedRead};

pub trait Sendable {
    fn format_message(&self) -> Message;
    fn send(&self) -> impl std::future::Future<Output = ()> + Send;
}

pub trait Emitable {
    fn format_event(&self) -> impl Serialize + Clone;
    fn emit(&self, handle: &AppHandle);
}

pub async fn upload_file(file: File) -> String {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:7331/upload")
        .body({
            let stream = FramedRead::new(file, BytesCodec::new());
            Body::wrap_stream(stream)
        })
        .send()
        .await
        .unwrap();

    response.text().await.unwrap()
}
