use log::debug;
use reqwest::{header::CONTENT_TYPE, Body};
use serde::Serialize;
use tauri::{
    http::{HeaderMap, HeaderValue},
    AppHandle, Emitter, Url,
};
use tauri_plugin_dialog::{DialogExt, FileResponse};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub fn pick_image(handle: &AppHandle) -> Option<FileResponse> {
    handle
        .dialog()
        .file()
        .add_filter("Images *.jpg *.jpeg", &["jpg", "jpeg"])
        .blocking_pick_file()
}

pub async fn upload_file(file: FileResponse) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    if let Some(mime_type) = file.mime_type {
        headers.insert(CONTENT_TYPE, HeaderValue::from_str(&mime_type).unwrap());
    }
    client
        .post("http://localhost:3000/upload")
        .headers(headers)
        .body({
            let stream = FramedRead::new(File::open(file.path).await.unwrap(), BytesCodec::new());
            Body::wrap_stream(stream)
        })
        .send()
        .await
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImagePayload {
    remote_path: Url,
}

pub fn handle_image(path: String, handle: AppHandle) {
    debug!("{}", path);
    let remote_path =
        Url::parse(format!("http://localhost:3000/uploads/{}", path).as_str()).unwrap();
    handle
        .emit("playImage", ImagePayload { remote_path })
        .unwrap();
}
