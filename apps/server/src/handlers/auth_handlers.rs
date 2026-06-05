use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use axum_valid::Garde;

use crate::{
    auth::{AuthUser, ExtractedSession, SESSION_COOKIE_NAME, SESSION_DURATION_HOURS},
    dto::{
        request::{SignInEmailParams, UpdatePreferencesParams},
        response::CurrentUserResponse,
    },
    error::AppResult,
    repositories::UserRepository,
    services::auth_service::AuthService,
    state::AppState,
};

fn real_client_ip(
    headers: &HeaderMap,
    connect_info: &ConnectInfo<SocketAddr>,
    trust_proxy: bool,
) -> IpAddr {
    if trust_proxy {
        if let Some(ip) = headers
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.split(',').next())
            .and_then(|s| s.trim().parse::<IpAddr>().ok())
        {
            return ip;
        }
        if let Some(ip) = headers
            .get("x-real-ip")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.trim().parse::<IpAddr>().ok())
        {
            return ip;
        }
    }
    connect_info.0.ip()
}

/// Whether the original client request reached us over HTTPS. The server always
/// listens on plain HTTP behind nginx, so we rely on the `X-Forwarded-Proto`
/// header (set by the proxy chain) when proxy headers are trusted. This decides
/// the session cookie's `Secure` flag: a `Secure` cookie is silently dropped by
/// browsers over HTTP, so we must not set it unless the request was HTTPS.
/// HTTPS deployments still get a hardened `Secure` cookie; plain-HTTP/LAN setups
/// can still log in.
fn request_is_secure(headers: &HeaderMap, trust_proxy: bool) -> bool {
    if !trust_proxy {
        return false;
    }
    headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().eq_ignore_ascii_case("https"))
        .unwrap_or(false)
}

pub async fn sign_in_email(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Extension(session): Extension<ExtractedSession>,
    Garde(Json(body)): Garde<Json<SignInEmailParams>>,
) -> AppResult<impl IntoResponse> {
    let ip = real_client_ip(&headers, &ConnectInfo(addr), state.trust_proxy_headers);
    let service = AuthService::new(state.db.clone());
    if let ExtractedSession::Valid(auth_user) = session {
        service.delete_session(&auth_user.session.id).await;
    }
    let session = service
        .sign_in_email(&body.email, &body.password, Some(ip))
        .await?;

    let max_age = SESSION_DURATION_HOURS * 60 * 60;
    let cookie = Cookie::build((SESSION_COOKIE_NAME, session.id))
        .path("/")
        .http_only(true)
        .secure(request_is_secure(&headers, state.trust_proxy_headers))
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .max_age(time::Duration::seconds(max_age as i64));

    Ok(CookieJar::new().add(cookie))
}

pub async fn sign_out(
    State(state): State<AppState>,
    headers: HeaderMap,
    auth_user: AuthUser,
) -> AppResult<impl IntoResponse> {
    let service = AuthService::new(state.db.clone());
    service.delete_session(&auth_user.session.id).await;

    let cookie = Cookie::build((SESSION_COOKIE_NAME, ""))
        .path("/")
        .http_only(true)
        .secure(request_is_secure(&headers, state.trust_proxy_headers))
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .max_age(time::Duration::ZERO);

    Ok(CookieJar::new().add(cookie))
}

pub async fn get_current_user(auth_user: AuthUser) -> Json<CurrentUserResponse> {
    Json(auth_user.user.into())
}

pub async fn update_preferences(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(body)): Garde<Json<UpdatePreferencesParams>>,
) -> AppResult<StatusCode> {
    let prefs =
        serde_json::to_value(body.preferences).expect("UserPreferences is always serializable");
    UserRepository::update_preferences(&state.db, auth_user.user.id, &prefs).await?;
    Ok(StatusCode::NO_CONTENT)
}
