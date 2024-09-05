use std::sync::Arc;

use futures_util::stream::{SplitSink, SplitStream};
use tokio::{net::TcpStream, sync::Mutex};
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
