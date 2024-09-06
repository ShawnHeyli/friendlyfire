use log::debug;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Url};
use tauri_plugin_dialog::{DialogExt, FileResponse};
use tokio_tungstenite::tungstenite::Message;

use crate::ws::messages::send_ws_message;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagePayload {
    remote_path: Url,
    text: String,
    width: f64,
    height: f64,
}

impl ImagePayload {
    pub fn new(remote_path: Url, text: String, width: f64, height: f64) -> Self {
        Self {
            remote_path,
            text,
            width,
            height,
        }
    }

    pub async fn send(&self) {
        debug!("Sent: {}", self.remote_path);
        send_ws_message(Message::Text(format!(
            "play_image;{};{};{};{}",
            self.remote_path, self.text, self.width, self.height
        )))
        .await
        .unwrap()
    }

    pub fn emit(&self, handle: &AppHandle) {
        debug!("Emit: {}", self.remote_path);
        if let Err(e) = handle.emit(
            "playImage",
            ImagePayload {
                remote_path: self.remote_path.clone(),
                text: self.text.clone(),
                width: self.width,
                height: self.height,
            },
        ) {
            log::error!("Failed to emit playImage event: {:?}", e);
        }
    }
}

pub fn pick_image(handle: &AppHandle) -> Option<FileResponse> {
    handle
        .dialog()
        .file()
        .add_filter(
            "Images *.BMP *.GIF *.JPEG *.PNG *.WebP *.SVG *.AVIF",
            &["bmp", "gif", "jpeg", "png", "wEBp", "svg", "avif"],
        )
        .blocking_pick_file()
}
