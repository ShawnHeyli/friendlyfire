use std::path::Path;

use image::ImageReader;
use log::debug;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Url};
use tokio_tungstenite::tungstenite::Message;

use crate::ws::messages::send_ws_message;

use super::{Emitable, Sendable};

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
}

impl Sendable for ImagePayload {
    fn format_message(&self) -> Message {
        Message::Text(format!(
            "play_image;{};{};{};{}",
            self.remote_path, self.text, self.width, self.height
        ))
    }

    async fn send(&self) {
        debug!("Sent: {}", self.remote_path);
        send_ws_message(self.format_message()).await.unwrap()
    }
}

impl Emitable for ImagePayload {
    fn format_event(&self) -> impl Serialize + Clone {
        ImagePayload {
            remote_path: self.remote_path.clone(),
            text: self.text.clone(),
            width: self.width,
            height: self.height,
        }
    }
    fn emit(&self, handle: &AppHandle) {
        debug!("Emit: {}", self.remote_path);
        if let Err(e) = handle.emit("playImage", self.format_event()) {
            log::error!("Failed to emit playImage event: {:?}", e);
        }
    }
}

pub fn dimensions(file_path: impl AsRef<Path>) -> Result<(f64, f64), image::ImageError> {
    let img = ImageReader::open(file_path).unwrap().decode()?;
    let width = img.width() as f64;
    let height = img.height() as f64;
    Ok((width, height))
}
