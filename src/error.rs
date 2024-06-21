use thiserror::Error;

#[derive(Error, Debug)]
pub enum HackNottsCheckinError {
    #[error("Failed to decode response: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Websocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
}
