use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum MbError {
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("HTTP status {0}")]
    Status(StatusCode),
}

pub type MbResult<T> = Result<T, MbError>;
