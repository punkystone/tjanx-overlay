use super::{get_ranking_error::GetRankingError, websocket_send_error::WebSocketSendError};
use tokio::sync::broadcast::error::SendError as BroadcastSendError;

pub enum SendError {
    WebSocketSendError(WebSocketSendError),
    GetRankingError(GetRankingError),
}

impl From<BroadcastSendError<String>> for SendError {
    fn from(inner: BroadcastSendError<String>) -> Self {
        Self::WebSocketSendError(WebSocketSendError { inner })
    }
}

impl From<GetRankingError> for SendError {
    fn from(inner: GetRankingError) -> Self {
        Self::GetRankingError(inner)
    }
}

impl std::fmt::Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SendError::WebSocketSendError(inner) => write!(f, "WebSocketSendError {inner}"),
            SendError::GetRankingError(inner) => write!(f, "GetRankingError {inner}"),
        }
    }
}
