use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("request failed: {0}")]
    Network(String),
    #[error("HTTP {status}")]
    Status { status: u16 },
    #[error("rate limited")]
    RateLimited,
    #[error("not found")]
    NotFound,
    #[error("response too large")]
    TooLarge,
    #[error("invalid response body: {0}")]
    InvalidBody(String),
}

impl HttpError {
    pub fn from_reqwest(err: reqwest::Error) -> Self {
        if err.is_timeout() || err.is_connect() || err.is_request() {
            Self::Network(err.to_string())
        } else {
            Self::Network(err.to_string())
        }
    }
}
