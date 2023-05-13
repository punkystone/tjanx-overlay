use std::string::FromUtf8Error;

pub enum GetRankingError {
    ClientNotInitialized,
    RequestBuilderError(hyper::http::Error),
    RequestError(hyper::Error),
    Utf8Error(FromUtf8Error),
}

impl From<hyper::Error> for GetRankingError {
    fn from(error: hyper::Error) -> Self {
        Self::RequestError(error)
    }
}

impl From<FromUtf8Error> for GetRankingError {
    fn from(error: FromUtf8Error) -> Self {
        Self::Utf8Error(error)
    }
}

impl From<hyper::http::Error> for GetRankingError {
    fn from(error: hyper::http::Error) -> Self {
        Self::RequestBuilderError(error)
    }
}

impl std::fmt::Display for GetRankingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetRankingError::ClientNotInitialized => {
                write!(f, "Client not initialized")
            }
            GetRankingError::RequestBuilderError(error) => {
                write!(f, "Request builder error: {error}")
            }
            GetRankingError::RequestError(error) => {
                write!(f, "Request error: {error}")
            }
            GetRankingError::Utf8Error(error) => {
                write!(f, "UTF-8 error: {error}")
            }
        }
    }
}
