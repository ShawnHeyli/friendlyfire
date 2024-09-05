use log::debug;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Url};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ImagePayload {
    remote_path: Url,
    text: String,
}

pub fn handle_image(path: String, text: String, handle: AppHandle) {
    debug!("{}", path);
    let remote_path =
        Url::parse(format!("http://localhost:3000/uploads/{}", path).as_str()).unwrap();
    handle
        .emit("playImage", ImagePayload { remote_path, text })
        .unwrap();
}
