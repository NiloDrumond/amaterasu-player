use amaterasu_macros::api_type;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[api_type("request/tag")]
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateTagParams {
    #[garde(length(min = 1, max = 64))]
    pub name: String,
    #[garde(inner(length(min = 1, max = 32)))]
    pub category: Option<String>,
    #[garde(inner(length(min = 1, max = 16)))]
    pub color: Option<String>,
}

/// PATCH semantics: any field present replaces the current value;
/// fields omitted from the request are left unchanged.
#[api_type("request/tag")]
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateTagParams {
    #[garde(inner(length(min = 1, max = 64)))]
    pub name: Option<String>,
    #[garde(inner(length(min = 1, max = 32)))]
    pub category: Option<String>,
    #[garde(inner(length(min = 1, max = 16)))]
    pub color: Option<String>,
}
