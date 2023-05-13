use tokio::sync::broadcast::error::SendError;


pub struct WebSocketSendError {
    pub inner: SendError<String>,
}

impl From<SendError<String>> for WebSocketSendError {
    fn from(inner: SendError<String>) -> Self {
        Self { inner }
    }
}

impl std::fmt::Display for WebSocketSendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WebSocketSendError: {}", self.inner)
    }
}
