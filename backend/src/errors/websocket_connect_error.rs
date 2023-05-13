use tokio_tungstenite::tungstenite::Error;

pub struct WebSocketConnectError {
    pub inner: Error,
}

impl From<Error> for WebSocketConnectError {
    fn from(inner: Error) -> Self {
        Self { inner }
    }
}

impl std::fmt::Display for WebSocketConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WebSocketConnectError: {}", self.inner)
    }
}
