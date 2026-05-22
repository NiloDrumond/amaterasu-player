use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub library_path: String,
    pub log_dir: String,
    pub data_dir: String,
    pub loki_url: Option<String>,
    pub grafana_url: Option<String>,
    pub admin_email: Option<String>,
    pub admin_password: Option<String>,
    pub admin_name: Option<String>,
    pub skip_initial_scan: bool,
    pub musicbrainz: MusicBrainzConfig,
}

#[derive(Debug, Clone)]
pub struct MusicBrainzConfig {
    pub enabled: bool,
    /// MB rejects unidentified clients; `Some(_)` whenever enabled.
    pub user_agent: Option<String>,
    pub base_url: String,
    pub cover_art_archive_base_url: String,
    pub max_candidates: usize,
}

const DEFAULT_MB_BASE_URL: &str = "https://musicbrainz.org/ws/2/";
const DEFAULT_CAA_BASE_URL: &str = "https://coverartarchive.org/";
const DEFAULT_MB_MAX_CANDIDATES: usize = 5;

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let mb_enabled = optional_env("MUSICBRAINZ_ENABLED")
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(true);
        let mb_user_agent = optional_env("MUSICBRAINZ_USER_AGENT");
        if mb_enabled && mb_user_agent.is_none() {
            anyhow::bail!(
                "MUSICBRAINZ_ENABLED=true but MUSICBRAINZ_USER_AGENT is not set. \
                 MusicBrainz rejects unidentified clients."
            );
        }

        let musicbrainz = MusicBrainzConfig {
            enabled: mb_enabled,
            user_agent: mb_user_agent,
            base_url: optional_env("MUSICBRAINZ_BASE_URL")
                .unwrap_or_else(|| DEFAULT_MB_BASE_URL.to_string()),
            cover_art_archive_base_url: optional_env("COVER_ART_ARCHIVE_BASE_URL")
                .unwrap_or_else(|| DEFAULT_CAA_BASE_URL.to_string()),
            max_candidates: optional_env("MUSICBRAINZ_MAX_CANDIDATES")
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_MB_MAX_CANDIDATES),
        };

        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST")?,
            server_port: env::var("SERVER_PORT")?.parse()?,
            library_path: env::var("LIBRARY_PATH")?,
            data_dir: env::var("DATA_DIR")?,
            log_dir: env::var("LOG_DIR").unwrap_or_else(|_| "./logs".to_string()),
            loki_url: optional_env("LOKI_URL"),
            grafana_url: optional_env("GRAFANA_URL"),
            admin_email: optional_env("ADMIN_EMAIL"),
            admin_password: optional_env("ADMIN_PASSWORD"),
            admin_name: optional_env("ADMIN_NAME"),
            skip_initial_scan: optional_env("SKIP_INITIAL_SCAN").unwrap_or_default() == "true",
            musicbrainz,
        })
    }
}

fn optional_env(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(val) if !val.trim().is_empty() => Some(val),
        _ => None,
    }
}
