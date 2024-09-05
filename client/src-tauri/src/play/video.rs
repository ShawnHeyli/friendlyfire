use log::debug;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Url};
use tauri_plugin_dialog::{DialogExt, FileResponse};
use tokio_tungstenite::tungstenite::Message;

use crate::ws::messages::send_ws_message;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoPayload {
    remote_path: Url,
    text: String,
}

impl VideoPayload {
    pub fn new(remote_path: Url, text: String) -> Self {
        Self { remote_path, text }
    }

    pub async fn send(&self) {
        debug!("{}", self.remote_path);
        send_ws_message(Message::Text(format!(
            "play_video;{};{}",
            self.remote_path, self.text
        )))
        .await
        .unwrap()
    }

    pub fn emit(&self, handle: &AppHandle) {
        debug!("{}", self.remote_path);
        if let Err(e) = handle.emit(
            "playVideo",
            VideoPayload {
                remote_path: self.remote_path.clone(),
                text: self.text.clone(),
            },
        ) {
            log::error!("Failed to emit playVideo event: {:?}", e);
        }
    }
}

pub fn pick_video(handle: &AppHandle) -> Option<FileResponse> {
    handle
        .dialog()
        .file()
        .add_filter(
            "Video *.3GP *.3G2 *.ASF *.AVI *.DivX *.M2V *.M3U *.M3U8 *.M4V *.MKV *.MOV *.MP4 *.MPEG *.OGV *.QVT *.RAM *.RM *.VOB *.WebM *.WMV *.XAP",
            &["3gp" ,"3g2" ,"asf" ,"avi" ,"dIVx" ,"m2v" ,"m3u" ,"m3u8" ,"m4v" ,"mkv" ,"mov" ,"mp4" ,"mpeg" ,"ogv" ,"qvt" ,"ram" ,"rm" ,"vob" ,"wEBm" ,"wmv" ,"xap"],
        )
        .blocking_pick_file()
}
