use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub library_path: String,
    pub log_dir: String,
    pub data_dir: String,
    pub admin_email: Option<String>,
    pub admin_password: Option<String>,
    pub admin_name: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST")?,
            server_port: env::var("SERVER_PORT")?.parse()?,
            library_path: env::var("LIBRARY_PATH")?,
            data_dir: env::var("DATA_DIR")?,
            log_dir: env::var("LOG_DIR").unwrap_or_else(|_| "./logs".to_string()),
            admin_email: optional_env("ADMIN_EMAIL"),
            admin_password: optional_env("ADMIN_PASSWORD"),
            admin_name: optional_env("ADMIN_NAME"),
        })
    }
}

fn optional_env(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(val) if !val.trim().is_empty() => Some(val),
        _ => None,
    }
}
