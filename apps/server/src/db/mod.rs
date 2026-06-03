pub mod entities;

use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    let options = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5));

    // Retry the initial connection. On container startup the database — or even
    // its DNS entry on the Docker network — may not be reachable yet; without
    // retries the process exits immediately and crash-loops instead of waiting.
    const MAX_ATTEMPTS: u32 = 30;
    let mut attempt = 1;
    loop {
        match options.clone().connect(database_url).await {
            Ok(pool) => return Ok(pool),
            Err(err) if attempt < MAX_ATTEMPTS => {
                tracing::warn!(
                    attempt,
                    max_attempts = MAX_ATTEMPTS,
                    error = %err,
                    "database not reachable yet, retrying in 2s"
                );
                tokio::time::sleep(Duration::from_secs(2)).await;
                attempt += 1;
            }
            Err(err) => return Err(err),
        }
    }
}
