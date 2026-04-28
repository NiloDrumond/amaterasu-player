use axum::{extract::State, http::StatusCode};

use crate::state::AppState;

pub async fn scan_library(State(state): State<AppState>) -> StatusCode {
    let Some(permit) = state.library_scanner.try_acquire_scan() else {
        return StatusCode::CONFLICT;
    };
    let scanner = state.library_scanner.clone();
    tokio::spawn(async move {
        if let Err(e) = scanner.run_scan(permit).await {
            tracing::warn!("Admin-triggered library scan failed: {}", e);
        }
    });
    StatusCode::ACCEPTED
}
