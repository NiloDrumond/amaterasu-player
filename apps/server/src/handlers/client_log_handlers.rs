//! Ingest endpoint for frontend (browser + SSR) logs.
//!
//! The browser cannot reach Loki directly (it is internal-only, never exposed
//! to clients), so the web app POSTs batches here and we forward them to Loki
//! as a dedicated `service=amaterasu-web` stream — separate from the server's
//! own `service=amaterasu-server` stream produced by the `tracing-loki` layer.
//! When `LOKI_URL` is unset (e.g. dev without the monitoring stack) we fall
//! back to the server's normal tracing output so errors are still visible.

use std::collections::BTreeMap;
use std::time::Duration;

use axum::{extract::State, http::StatusCode, Extension, Json};
use serde::Deserialize;
use serde_json::{json, Map, Value};

use crate::{auth::ExtractedSession, state::AppState};

/// Hard cap on how many entries a single batch may contribute, to bound work
/// and Loki payload size regardless of what a client sends.
const MAX_ENTRIES: usize = 100;
/// Per-field character cap for free-form text (message/stack) before truncation.
const MAX_TEXT_LEN: usize = 8192;

/// Pushes frontend log batches to Loki's HTTP push API. Mirrors the small
/// `GrafanaProxy` client pattern: a reused `reqwest::Client` + a base URL.
#[derive(Debug, Clone)]
pub struct LokiIngest {
    push_url: String,
    env: String,
    client: reqwest::Client,
}

impl LokiIngest {
    pub fn new(loki_url: String, env: String) -> Self {
        let base = loki_url.trim_end_matches('/');
        let push_url = format!("{base}/loki/api/v1/push");
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("failed to build reqwest client for loki ingest");
        Self {
            push_url,
            env,
            client,
        }
    }

    async fn push(
        &self,
        entries: &[ClientLogEntry],
        identity: &Identity,
    ) -> anyhow::Result<()> {
        // `level` is a stream label, so values must be grouped by level.
        let mut by_level: BTreeMap<&'static str, Vec<[String; 2]>> = BTreeMap::new();

        for entry in entries {
            let level = normalize_level(entry.level.as_deref());
            let ts_nanos = entry
                .ts
                .filter(|ms| *ms > 0)
                .map(|ms| ms as i128 * 1_000_000)
                .unwrap_or_else(now_nanos);
            let line = build_line(entry, identity);
            by_level
                .entry(level)
                .or_default()
                .push([ts_nanos.to_string(), line]);
        }

        let streams: Vec<Value> = by_level
            .into_iter()
            .map(|(level, values)| {
                json!({
                    "stream": {
                        "service": "amaterasu-web",
                        "level": level,
                        "env": self.env,
                    },
                    "values": values,
                })
            })
            .collect();

        let res = self
            .client
            .post(&self.push_url)
            .json(&json!({ "streams": streams }))
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            anyhow::bail!("loki responded {status}: {body}");
        }
        Ok(())
    }
}

/// One log entry as sent by the web app. All fields beyond `message` are
/// optional so the client stays cheap and forward-compatible.
#[derive(Debug, Deserialize)]
pub struct ClientLogEntry {
    pub level: Option<String>,
    pub message: String,
    pub stack: Option<String>,
    pub url: Option<String>,
    pub route: Option<String>,
    /// Client epoch milliseconds; falls back to receive time when absent.
    pub ts: Option<i64>,
    pub context: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct ClientLogBatch {
    #[serde(default)]
    pub entries: Vec<ClientLogEntry>,
}

/// Server-trusted identity derived from the session cookie — preferred over any
/// client-supplied identity, which could be forged.
struct Identity {
    user_id: Option<String>,
    user_email: Option<String>,
}

pub async fn ingest(
    State(state): State<AppState>,
    Extension(session): Extension<ExtractedSession>,
    Json(batch): Json<ClientLogBatch>,
) -> StatusCode {
    if batch.entries.is_empty() {
        return StatusCode::ACCEPTED;
    }

    let identity = match &session {
        ExtractedSession::Valid(auth_user) => Identity {
            user_id: Some(auth_user.user.id.to_string()),
            user_email: Some(auth_user.user.email.clone()),
        },
        ExtractedSession::Invalid(_) => Identity {
            user_id: None,
            user_email: None,
        },
    };

    let entries: Vec<ClientLogEntry> = batch.entries.into_iter().take(MAX_ENTRIES).collect();

    match state.client_log.as_ref() {
        Some(ingest) => {
            if let Err(e) = ingest.push(&entries, &identity).await {
                // Telemetry must never fail the client: log and accept anyway.
                tracing::warn!("client-log: loki push failed: {e}");
            }
        }
        None => {
            for entry in &entries {
                emit_local(entry, &identity);
            }
        }
    }

    StatusCode::ACCEPTED
}

/// Fallback when no Loki is configured: surface frontend logs in the server's
/// own tracing output. The `amaterasu_server::client_log` target matches the
/// default `amaterasu_server=debug` env filter.
fn emit_local(entry: &ClientLogEntry, identity: &Identity) {
    let route = entry.route.as_deref().unwrap_or("-");
    let user = identity.user_id.as_deref().unwrap_or("-");
    let stack = entry.stack.as_deref().unwrap_or("");
    match normalize_level(entry.level.as_deref()) {
        "info" => {
            tracing::info!(target: "amaterasu_server::client_log", route, user, "{}", entry.message)
        }
        "warn" => {
            tracing::warn!(target: "amaterasu_server::client_log", route, user, "{} {}", entry.message, stack)
        }
        _ => {
            tracing::error!(target: "amaterasu_server::client_log", route, user, "{} {}", entry.message, stack)
        }
    }
}

/// Builds the JSON log line so Grafana's `{service="amaterasu-web"} | json`
/// parses each field. Caller-supplied `context` keys are merged in but never
/// override the trusted/structured fields.
fn build_line(entry: &ClientLogEntry, identity: &Identity) -> String {
    let mut obj = Map::new();
    obj.insert("message".into(), json!(truncate(&entry.message)));
    if let Some(stack) = &entry.stack {
        obj.insert("stack".into(), json!(truncate(stack)));
    }
    if let Some(url) = &entry.url {
        obj.insert("url".into(), json!(url));
    }
    if let Some(route) = &entry.route {
        obj.insert("route".into(), json!(route));
    }
    if let Some(user_id) = &identity.user_id {
        obj.insert("userId".into(), json!(user_id));
    }
    if let Some(email) = &identity.user_email {
        obj.insert("userEmail".into(), json!(email));
    }

    match &entry.context {
        Some(Value::Object(ctx)) => {
            for (k, v) in ctx {
                obj.entry(k.clone()).or_insert_with(|| v.clone());
            }
        }
        Some(other) => {
            obj.insert("context".into(), other.clone());
        }
        None => {}
    }

    Value::Object(obj).to_string()
}

fn normalize_level(level: Option<&str>) -> &'static str {
    match level {
        Some(l) if l.eq_ignore_ascii_case("info") => "info",
        Some(l) if l.eq_ignore_ascii_case("warn") || l.eq_ignore_ascii_case("warning") => "warn",
        _ => "error",
    }
}

fn now_nanos() -> i128 {
    chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as i128
}

fn truncate(s: &str) -> String {
    if s.len() <= MAX_TEXT_LEN {
        return s.to_string();
    }
    let mut end = MAX_TEXT_LEN;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &s[..end])
}
