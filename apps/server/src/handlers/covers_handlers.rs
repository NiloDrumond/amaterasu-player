use axum::{
    extract::{Path, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use axum_valid::Garde;
use tokio::fs;

use crate::{
    dto::request::CoverPath,
    error::{AppError, AppResult},
    state::AppState,
};

pub async fn get_cover(
    State(state): State<AppState>,
    Garde(Path(CoverPath { filename })): Garde<Path<CoverPath>>,
) -> AppResult<Response> {
    let path = state.covers_dir.join(&filename);
    let bytes = fs::read(&path).await.map_err(|_| AppError::NotFound)?;

    let ext = filename.rsplit_once('.').map(|(_, e)| e).unwrap_or("");
    let mime = match ext {
        "jpg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "application/octet-stream",
    };

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, HeaderValue::from_static(mime)),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=31536000, immutable"),
            ),
        ],
        bytes,
    )
        .into_response())
}
