use std::future::Future;

use crate::{state::AppState, tasks::auth_tasks::delete_expired_sessions_tasks};

mod auth_tasks;

pub fn initialize_background_tasks(state: AppState) -> impl Future {
    delete_expired_sessions_tasks(state)
}
