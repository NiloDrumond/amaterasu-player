use tokio::time::sleep;

use crate::{auth::SESSION_CLEANUP_INTERVAL, services::auth_service::AuthService, state::AppState};

pub async fn delete_expired_sessions_tasks(state: AppState) {
    loop {
        let service = AuthService::new(state.db.clone());
        if let Err(e) = service.delete_expired().await {
            tracing::warn!("Failed to delete expired sessions: {e}");
        }
        sleep(SESSION_CLEANUP_INTERVAL).await;
    }
}
