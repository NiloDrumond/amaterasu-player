use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

use crate::{auth::middleware::AdminUser, error::AppError, state::AppState};

#[derive(Debug, Clone)]
pub struct GrafanaProxy {
    base_url: String,
    client: reqwest::Client,
}

impl GrafanaProxy {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("failed to build reqwest client for grafana proxy");
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        }
    }
}

fn is_hop_by_hop(name: &HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailers"
            | "transfer-encoding"
            | "upgrade"
            | "host"
            | "content-length"
    )
}

pub async fn proxy(
    State(state): State<AppState>,
    admin: AdminUser,
    req: Request,
) -> Result<Response, AppError> {
    let Some(proxy) = state.grafana_proxy.as_ref() else {
        return Ok((StatusCode::SERVICE_UNAVAILABLE, "Grafana proxy disabled").into_response());
    };

    // Grafana is configured with GF_SERVER_SERVE_FROM_SUB_PATH=true, so it only
    // responds at /admin/logs/*. Routes are registered at the top level (no nest),
    // so req.uri().path() already includes the full /admin/logs/... prefix.
    let path = req.uri().path();
    let query = req.uri().query();
    let target = match query {
        Some(q) => format!("{}{}?{}", proxy.base_url, path, q),
        None => format!("{}{}", proxy.base_url, path),
    };

    let method = reqwest::Method::from_bytes(req.method().as_str().as_bytes())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("bad method: {e}")))?;

    let mut headers = HeaderMap::new();
    for (name, value) in req.headers() {
        if !is_hop_by_hop(name) {
            headers.insert(name.clone(), value.clone());
        }
    }
    let user = &admin.0.user;
    if let Ok(v) = HeaderValue::from_str(&user.email) {
        headers.insert(HeaderName::from_static("x-webauth-user"), v.clone());
        headers.insert(HeaderName::from_static("x-webauth-email"), v);
    }
    if let Ok(v) = HeaderValue::from_str(&user.name) {
        headers.insert(HeaderName::from_static("x-webauth-name"), v);
    }

    // Collect the request body to bytes so reqwest can set Content-Length.
    // Streaming with wrap_stream forces chunked encoding even on GETs, which
    // makes some upstreams (incl. Grafana) wait indefinitely for a body.
    // 32 MiB is plenty for dashboard CRUD / queries.
    let body_bytes = axum::body::to_bytes(req.into_body(), 32 * 1024 * 1024)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("read request body: {e}")))?;

    let upstream = proxy
        .client
        .request(method, &target)
        .headers(headers)
        .body(body_bytes)
        .send()
        .await
        .map_err(|e| {
            tracing::warn!("grafana proxy upstream error: {e}");
            AppError::Internal(anyhow::anyhow!("grafana upstream error: {e}"))
        })?;

    let status = StatusCode::from_u16(upstream.status().as_u16())
        .map_err(|e| AppError::Internal(anyhow::anyhow!("bad status: {e}")))?;
    let upstream_headers = upstream.headers().clone();

    let stream = upstream.bytes_stream();
    let body = Body::from_stream(stream);

    let mut response = Response::new(body);
    *response.status_mut() = status;
    let response_headers = response.headers_mut();
    for (name, value) in upstream_headers.iter() {
        if !is_hop_by_hop(name) {
            response_headers.insert(name.clone(), value.clone());
        }
    }
    Ok(response)
}
