use tokio::time::sleep;

use crate::{
    auth::SESSION_CLEANUP_INTERVAL, error::AppResult, services::auth_service::AuthService,
    state::AppState,
};

pub async fn delete_expired_sessions_tasks(state: AppState) -> AppResult<()> {
    loop {
        let service = AuthService::new(state.db.clone());
        service.delete_expired().await?;
        sleep(SESSION_CLEANUP_INTERVAL).await;
    }
}
