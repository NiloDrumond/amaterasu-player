use amaterasu_macros::api_type;
use serde::Deserialize;

#[api_type("request/common")]
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default)]
    pub offset: i32,
}

fn default_limit() -> i32 {
    32
}
