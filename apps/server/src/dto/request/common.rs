use amaterasu_macros::api_type;
use serde::Deserialize;

#[api_type("request/common")]
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    32
}
