pub mod image;
pub mod video;

use reqwest::{header::CONTENT_TYPE, Body};
use tauri::http::{HeaderMap, HeaderValue};
use tauri_plugin_dialog::FileResponse;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn upload_file(file: FileResponse) -> String {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    if let Some(mime_type) = file.mime_type {
        headers.insert(CONTENT_TYPE, HeaderValue::from_str(&mime_type).unwrap());
    }
    let response = client
        .post("http://localhost:3000/upload")
        .headers(headers)
        .body({
            let stream = FramedRead::new(File::open(file.path).await.unwrap(), BytesCodec::new());
            Body::wrap_stream(stream)
        })
        .send()
        .await
        .unwrap();
    response.text().await.unwrap()
}
