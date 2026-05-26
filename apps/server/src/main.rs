use std::net::SocketAddr;
use std::time::Duration;

use tokio::signal;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod db;
mod dto;
mod error;
mod filters;
pub mod handlers;
mod musicbrainz;
mod repositories;
mod routes;
mod scanner;
mod search;
mod services;
mod state;
mod streaming;
mod tasks;

use config::Config;
use state::AppState;

use crate::services::auth_service::AuthService;
use crate::tasks::initialize_background_tasks;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let file_appender = tracing_appender::rolling::daily(&config.log_dir, "amaterasu-server");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let loki_layer = if let Some(url) = &config.loki_url {
        let (layer, task) = tracing_loki::builder()
            .label("service", "amaterasu-server")?
            .label(
                "env",
                std::env::var("APP_ENV").unwrap_or_else(|_| "dev".into()),
            )?
            .build_url(url.parse()?)?;
        tokio::spawn(task);
        Some(layer)
    } else {
        None
    };

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
        .with(loki_layer)
        .init();
    tracing::info!(
        "Starting server on {}:{}",
        config.server_host,
        config.server_port
    );

    if !std::path::Path::new(&config.library_path).is_dir() {
        tracing::warn!(
            "LIBRARY_PATH '{}' does not exist or is not a directory",
            config.library_path
        );
    }

    let db_pool = db::create_pool(&config.database_url, config.database_max_connections).await?;
    tracing::info!("Database connected");

    bootstrap_admin(&db_pool, &config).await?;

    let covers_dir = std::path::PathBuf::from(&config.data_dir).join("covers");
    std::fs::create_dir_all(&covers_dir)?;

    let search_dir = std::path::PathBuf::from(&config.data_dir).join("search-index");
    let search_index = std::sync::Arc::new(search::SearchIndex::open(&search_dir)?);
    match search::sync::rebuild_from_postgres(&db_pool, &search_index).await {
        Ok(counts) => tracing::info!(
            "search: rebuilt index (tracks={}, albums={}, artists={}, playlists={}, collections={})",
            counts.tracks,
            counts.albums,
            counts.artists,
            counts.playlists,
            counts.collections
        ),
        Err(e) => tracing::warn!("search: initial rebuild failed: {}", e),
    }

    // Construct the MB suggestion service + spawn the worker before the
    // scanner so we can hand it the lookup-sender on construction.
    let (mb_service, mb_lookup_sender) = if config.musicbrainz.enabled {
        let user_agent = config
            .musicbrainz
            .user_agent
            .as_deref()
            .expect("user_agent presence checked in Config::from_env");
        let mb_client =
            musicbrainz::MusicBrainzClient::new(user_agent, config.musicbrainz.base_url.clone())?;
        let caa_client = musicbrainz::CoverArtClient::new(
            user_agent,
            config.musicbrainz.cover_art_archive_base_url.clone(),
        )?;
        let service = musicbrainz::MetadataSuggestionService::new(
            db_pool.clone(),
            covers_dir.clone(),
            mb_client,
            caa_client,
            config.musicbrainz.max_candidates,
        );
        let sender = musicbrainz::spawn_worker(service.clone());
        (Some(service), Some(sender))
    } else {
        tracing::info!("MusicBrainz integration disabled (MUSICBRAINZ_ENABLED=false)");
        (None, None)
    };

    let mut library_scanner =
        scanner::LibraryScanner::new(config.library_path, covers_dir.clone(), db_pool.clone());
    if let Some(sender) = mb_lookup_sender.clone() {
        library_scanner = library_scanner.with_mb_lookup_sender(sender);
    }

    if !config.skip_initial_scan {
        let library_scanner_clone = library_scanner.clone();
        let post_scan_pool = db_pool.clone();
        let post_scan_search = search_index.clone();
        tokio::spawn(async move {
            if let Err(e) = library_scanner_clone.scan_library().await {
                tracing::warn!("Failed to scan music library: {}", e);
                tracing::warn!(
                    "Server will continue running, but library may not be fully indexed"
                );
            }
            // Scanner mutations bypass write-through; rebuild the search index from
            // Postgres so newly scanned tracks/albums/artists become searchable.
            match search::sync::rebuild_from_postgres(&post_scan_pool, &post_scan_search).await {
                Ok(c) => tracing::info!(
                    "search: rebuilt after scan (tracks={}, albums={}, artists={})",
                    c.tracks,
                    c.albums,
                    c.artists
                ),
                Err(e) => tracing::warn!("search: post-scan rebuild failed: {}", e),
            }
        });
    }

    let grafana_proxy = config
        .grafana_url
        .as_ref()
        .map(|url| handlers::grafana_proxy_handlers::GrafanaProxy::new(url.clone()));

    let app_state = AppState::new(
        db_pool,
        library_scanner,
        covers_dir,
        search_index,
        grafana_proxy,
        mb_service,
        mb_lookup_sender,
        config.trust_proxy_headers,
    );

    let tasks_state = app_state.clone();
    tokio::spawn(async {
        initialize_background_tasks(tasks_state).await;
    });

    let auth_governor = GovernorConfigBuilder::default()
        .key_extractor(SmartIpKeyExtractor)
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();
    let protected_governor = GovernorConfigBuilder::default()
        .key_extractor(SmartIpKeyExtractor)
        .per_second(30)
        .burst_size(60)
        .finish()
        .unwrap();
    let auth_limiter = auth_governor.limiter().clone();
    let protected_limiter = protected_governor.limiter().clone();
    let interval = Duration::from_secs(60);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            auth_limiter.retain_recent();
            protected_limiter.retain_recent();
        }
    });

    let app = routes::create_api_router(
        app_state,
        auth_governor,
        protected_governor,
        config.cors_allowed_origins.as_deref(),
    );

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
            .await?;

    tracing::info!(
        "Server listening on {}:{}",
        config.server_host,
        config.server_port
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}

async fn bootstrap_admin(pool: &sqlx::PgPool, config: &Config) -> anyhow::Result<()> {
    match (
        config.admin_email.as_deref(),
        config.admin_password.as_deref(),
    ) {
        (Some(email), Some(password)) => {
            let name = config.admin_name.as_deref().unwrap_or("Admin");
            AuthService::new(pool.clone())
                .bootstrap_admin(email, password, name)
                .await?;
            Ok(())
        }
        (None, None) => {
            tracing::info!("ADMIN_EMAIL and ADMIN_PASSWORD not set, skipping admin bootstrap");
            Ok(())
        }
        _ => Err(anyhow::anyhow!(
            "ADMIN_EMAIL and ADMIN_PASSWORD must both be set or both unset"
        )),
    }
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
