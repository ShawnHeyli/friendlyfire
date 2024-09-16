use std::fmt;
use std::sync::Arc;

use futures_util::stream::{SplitSink, SplitStream};
use serde::Serialize;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::Error as TungsteniteError;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub mod close;
pub mod init;
pub mod messages;

type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WebSocketSplitSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WebSocketSplitStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

lazy_static::lazy_static! {
    static ref WS_CONNECTION: Arc<Mutex<Option<WebSocketSplitSink>>> = Arc::new(Mutex::new(None));
}

#[derive(Debug)]
pub enum WebSocketError {
    ConnectionError(TungsteniteError),
    SendError(TungsteniteError),
    ReceiveError(TungsteniteError),
    ParseError(&'static str),
    UnknownError,
}

impl Serialize for WebSocketError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            WebSocketError::ConnectionError(e) => {
                serializer.serialize_str(&format!("ConnectionError: {}", e))
            }
            WebSocketError::SendError(e) => serializer.serialize_str(&format!("SendError: {}", e)),
            WebSocketError::ReceiveError(e) => {
                serializer.serialize_str(&format!("ReceiveError: {}", e))
            }
            WebSocketError::ParseError(e) => {
                serializer.serialize_str(&format!("ParseError: {}", e))
            }
            WebSocketError::UnknownError => serializer.serialize_str("UnknownError"),
        }
    }
}

impl fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebSocketError::ConnectionError(e) => write!(f, "WebSocket connection error: {}", e),
            WebSocketError::SendError(e) => write!(f, "WebSocket send error: {}", e),
            WebSocketError::ReceiveError(e) => write!(f, "WebSocket receive error: {}", e),
            WebSocketError::ParseError(e) => write!(f, "Parse error: {}", e),
            WebSocketError::UnknownError => write!(f, "Unknown error"),
        }
    }
}
