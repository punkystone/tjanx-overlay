use super::{get_ranking_error::GetRankingError, websocket_connect_error::WebSocketConnectError};

pub enum WebSocketConnectionError {
    WebSocketConnectError(WebSocketConnectError),
    GetRankingError(GetRankingError),
}

impl From<WebSocketConnectError> for WebSocketConnectionError {
    fn from(inner: WebSocketConnectError) -> Self {
        Self::WebSocketConnectError(inner)
    }
}

impl From<GetRankingError> for WebSocketConnectionError {
    fn from(inner: GetRankingError) -> Self {
        Self::GetRankingError(inner)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for WebSocketConnectionError {
    fn from(inner: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WebSocketConnectError(WebSocketConnectError { inner })
    }
}

impl std::fmt::Display for WebSocketConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebSocketConnectionError::WebSocketConnectError(inner) => {
                write!(f, "WebSocketConnectError {inner}")
            }
            WebSocketConnectionError::GetRankingError(inner) => {
                write!(f, "GetRankingError {inner}")
            }
        }
    }
}
