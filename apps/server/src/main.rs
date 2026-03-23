use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod dto;
mod error;
mod handlers;
mod repositories;
mod routes;
mod scanner;
mod services;
mod state;
mod utils;
mod auth;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let file_appender =
        tracing_appender::rolling::daily(&config.log_dir, "amaterasu-server");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "amaterasu_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false),
        )
        .init();
    tracing::info!(
        "Starting server on {}:{}",
        config.server_host,
        config.server_port
    );

    let db_pool = db::create_pool(&config.database_url).await?;
    tracing::info!("Database connected");

    let library_scanner = scanner::LibraryScanner::new(config.library_path, db_pool.clone());

    // Try to scan the library but don't crash if it fails
    if let Err(e) = library_scanner.scan_library().await {
        tracing::warn!("Failed to scan music library: {}", e);
        tracing::warn!("Server will continue running, but library may not be fully indexed");
    }

    let app_state = AppState::new(db_pool, library_scanner);

    let app = routes::create_api_router().with_state(app_state);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
            .await?;

    tracing::info!(
        "Server listening on {}:{}",
        config.server_host,
        config.server_port
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
