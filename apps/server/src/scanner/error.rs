#[derive(Debug, thiserror::Error)]
pub enum ScannerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Symphonia error: {0}")]
    Symphonia(#[from] symphonia::core::errors::Error),

    #[error("Failed to extract metadata")]
    FailedToExtractMetadata,

    #[error("Failed to detect format")]
    FailedToDetectFormat,

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

pub type ScannerResult<T> = Result<T, ScannerError>;
