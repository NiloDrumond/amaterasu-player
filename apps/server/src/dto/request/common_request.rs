use amaterasu_macros::api_type;
use serde::Deserialize;

use crate::filters::FilterNode;

fn default_limit() -> i32 {
    32
}

#[api_type("request/common")]
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum SortDir {
    Asc,
    Desc,
}

impl SortDir {
    pub fn as_sql(self) -> &'static str {
        match self {
            SortDir::Asc => "ASC",
            SortDir::Desc => "DESC",
        }
    }
}

#[api_type("request/common")]
#[derive(Debug, Deserialize)]
pub struct SearchPaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default)]
    pub offset: i32,
    /// Optional case-insensitive `q` text query.
    #[serde(default)]
    pub q: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub dir: Option<SortDir>,
}

#[api_type("request/common")]
#[derive(Debug, Deserialize)]
pub struct FilteredPaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default)]
    pub offset: i32,
    /// base64url-encoded JSON FilterNode
    #[serde(default)]
    pub f: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub dir: Option<SortDir>,
}

impl FilteredPaginationParams {
    pub fn decode_filter(&self) -> Result<Option<FilterNode>, crate::error::AppError> {
        let Some(raw) = self.f.as_deref() else {
            return Ok(None);
        };
        if raw.is_empty() {
            return Ok(None);
        }
        decode_filter_param(raw).map(Some)
    }
}

pub fn decode_filter_param(raw: &str) -> Result<FilterNode, crate::error::AppError> {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    let bytes = URL_SAFE_NO_PAD
        .decode(raw)
        .map_err(|e| crate::error::AppError::BadRequest(format!("invalid filter encoding: {e}")))?;
    let node: FilterNode = serde_json::from_slice(&bytes)
        .map_err(|e| crate::error::AppError::BadRequest(format!("invalid filter json: {e}")))?;
    Ok(node)
}
