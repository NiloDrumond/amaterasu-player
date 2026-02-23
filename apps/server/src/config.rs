use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub library_path: String,
    pub log_dir: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST")?,
            server_port: env::var("SERVER_PORT")?.parse()?,
            library_path: env::var("LIBRARY_PATH")?,
            log_dir: env::var("LOG_DIR").unwrap_or_else(|_| "./logs".to_string()),
        })
    }
}

