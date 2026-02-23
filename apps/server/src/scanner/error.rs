#[derive(Debug, thiserror::Error)]
pub enum ScannerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("FFmpeg error: {0}")]
    FFmpeg(#[from] ffmpeg_next::Error),

    #[error("Failed to extract metadata")]
    FailedToExtractMetadata,


    #[error("Invalid file name")]
    InvalidFileName(Option<String>),

    #[error("Failed to detect format")]
    FailedToDetectFormat,

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

pub type ScannerResult<T> = Result<T, ScannerError>;
