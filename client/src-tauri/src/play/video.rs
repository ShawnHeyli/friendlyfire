use ffmpeg_next::{self as ffmpeg, media::Type};
use std::path::Path;

use ffmpeg_next::format::input;
use log::debug;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Url};
use tokio_tungstenite::tungstenite::Message;

use crate::ws::messages::send_ws_message;

use super::{Emitable, Sendable};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPayload {
    remote_path: Url,
    text: String,
    width: f64,
    height: f64,
}

impl VideoPayload {
    pub fn new(remote_path: Url, text: String, height: f64, width: f64) -> Self {
        Self {
            remote_path,
            text,
            width,
            height,
        }
    }
}

impl Sendable for VideoPayload {
    fn format_message(&self) -> Message {
        Message::Text(format!(
            "play_video;{};{};{};{}",
            self.remote_path, self.text, self.width, self.height
        ))
    }

    async fn send(&self) {
        debug!("{}", self.remote_path);
        send_ws_message(self.format_message()).await.unwrap()
    }
}

impl Emitable for VideoPayload {
    fn format_event(&self) -> impl Serialize + Clone {
        VideoPayload {
            remote_path: self.remote_path.clone(),
            text: self.text.clone(),
            width: self.width,
            height: self.height,
        }
    }

    fn emit(&self, handle: &AppHandle) {
        debug!("{}", self.remote_path);
        if let Err(e) = handle.emit("playVideo", self.format_event()) {
            log::error!("Failed to emit playVideo event: {:?}", e);
        }
    }
}

pub fn dimensions(file_path: impl AsRef<Path>) -> Result<(f64, f64), ffmpeg::Error> {
    ffmpeg::init()?;

    let ictx = input(&file_path)?;
    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    let decoder = context_decoder.decoder().video()?;

    Ok((decoder.width() as f64, decoder.height() as f64))
}
