use std::borrow::Cow;

use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

use super::{messages::send_ws_message, WebSocketError, WS_CONNECTION};

pub async fn close_ws_connection() -> Result<(), WebSocketError> {
    if WS_CONNECTION.lock().await.is_some() {
        send_ws_message(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::Borrowed("User disconnecting"),
        })))
        .await?;

        let mut ws_connection = WS_CONNECTION.lock().await;
        *ws_connection = None;
    }
    Ok(())
}
