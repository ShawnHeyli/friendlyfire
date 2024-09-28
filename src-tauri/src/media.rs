use image::{DynamicImage, ImageFormat, ImageReader};
use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tokio::{fs::File, io::AsyncReadExt, sync::Mutex};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    message::{WsError, WsMessage},
    server::ServerState,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaMessage {
    media_url: tauri::Url,
    top_message: String,
    bottom_message: String,
    sender: User,
    timeout: u64, // in milliseconds
}

impl From<MediaMessage> for Message {
    fn from(val: MediaMessage) -> Self {
        Message::Text(serde_json::to_string(&val).unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    // avatar_url: Option<tauri::Url>,
    username: String,
}

#[derive(Debug)]
pub enum SendError {
    Io(tokio::io::Error),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Image(image::ImageError),
    Webp(String),
    Tungstenite(tokio_tungstenite::tungstenite::Error),
    SendingError(WsError),
    RemoteMediaNone,
    UrlNone,
}

impl From<tokio::io::Error> for SendError {
    fn from(err: tokio::io::Error) -> Self {
        SendError::Io(err)
    }
}

impl From<reqwest::Error> for SendError {
    fn from(err: reqwest::Error) -> Self {
        SendError::Reqwest(err)
    }
}

impl From<serde_json::Error> for SendError {
    fn from(err: serde_json::Error) -> Self {
        SendError::Serde(err)
    }
}

impl From<image::ImageError> for SendError {
    fn from(err: image::ImageError) -> Self {
        SendError::Image(err)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for SendError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        SendError::Tungstenite(err)
    }
}

impl From<WsError> for SendError {
    fn from(err: WsError) -> Self {
        SendError::SendingError(err)
    }
}

impl Display for SendError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SendError::Io(err) => write!(f, "IO error: {}", err),
            SendError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            SendError::Serde(err) => write!(f, "Serde error: {}", err),
            SendError::Image(err) => write!(f, "Image error: {}", err),
            SendError::Webp(err) => write!(f, "WebP encoding error: {:?}", err),
            SendError::Tungstenite(err) => write!(f, "Tungstenite error: {:?}", err),
            SendError::SendingError(error) => write!(f, "{}", error),
            SendError::RemoteMediaNone => write!(f, "No remote media url"),
            SendError::UrlNone => write!(f, "No URL"),
        }
    }
}

pub async fn send(
    handle: AppHandle,
    filepath: PathBuf,
    top_message: String,
    bottom_message: String,
    user: User,
    timeout: u64,
) -> Result<(), SendError> {
    let image_reader = ImageReader::open(filepath.clone())?.with_guessed_format()?;
    let img_data = match image_reader.format() {
        Some(ImageFormat::Gif) => {
            let mut image_file = File::open(filepath).await?;
            let mut buf = Vec::new();
            image_file.read_to_end(&mut buf).await?;
            buf
        }
        Some(_) => {
            let decoded_image = image_reader.decode()?;
            encode_webp(&decoded_image)?
        }
        None => {
            let format_hint = image::error::ImageFormatHint::PathExtension(filepath);
            return Err(SendError::Image(image::ImageError::Unsupported(
                image::error::UnsupportedError::from_format_and_kind(
                    format_hint.clone(),
                    image::error::UnsupportedErrorKind::Format(format_hint),
                ),
            )));
        }
    };

    let mutex_state = handle.state::<Mutex<ServerState>>();
    let state = mutex_state.lock().await;

    if let Some(url) = state.upload_url().clone() {
        let client = reqwest::Client::new();
        let res = client.post(url).body(img_data).send().await?;
        let remote_path = res.text().await?;

        // File uploaded on the server from now

        if let Some(remote_path) = state.remote_media(remote_path) {
            let message = WsMessage::Media(MediaMessage {
                media_url: remote_path,
                top_message,
                bottom_message,
                sender: user,
                timeout,
            });
            message.send(&handle).await?;
            return Ok(());
        }
        return Err(SendError::RemoteMediaNone);
    }
    Err(SendError::UrlNone)
}

fn encode_webp(image: &DynamicImage) -> Result<Vec<u8>, SendError> {
    let encoder = webp::Encoder::from_image(image).map_err(|e| SendError::Webp(e.to_string()))?;
    let webp_data = encoder.encode(60.0);
    Ok(webp_data.to_vec())
}
