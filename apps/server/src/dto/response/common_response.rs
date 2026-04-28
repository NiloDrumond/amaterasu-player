use amaterasu_macros::api_type;
use serde::Serialize;

#[api_type("response/common")]
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub limit: i32,
    pub offset: i32,
}
