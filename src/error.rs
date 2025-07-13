use thiserror::Error;

#[derive(Error, Debug)]
pub enum XrplError {
    #[error("XRPL API returned error: {error}: {}",
        error_message.as_ref().map(|msg| msg.to_string())
        .unwrap_or_default())]
    ApiError { error: String, error_message: Option<String> },
    #[error("Failed to parse XRPL response: {0}")]
    ParseError(String),
    #[error("Socket error: {0}")]
    SocketError(#[from] XrplSocketError),
}

#[derive(Error, Debug)]
pub enum XrplSocketError {
    #[error("Failed to connect to WebSocket: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("WebSocket connection closed unexpectedly")]
    ConnectionClosed,

    #[error("Request timed out after {timeout_ms}ms")]
    RequestTimeout { timeout_ms: u64 },

    #[error("Failed to send message over internal channel")]
    ChannelSendError,

    #[error("Failed to receive response from internal channel")]
    ChannelReceiveError,

    #[error("Failed to parse JSON response: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("WebSocket is disconnected")]
    Disconnected,

    #[error("Invalid request format: missing required field '{field}'")]
    InvalidRequest { field: String },

    #[error("WebSocket ping failed")]
    PingFailed,
}
