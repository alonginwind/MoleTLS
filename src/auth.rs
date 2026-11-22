// Basic-auth protection for the admin API (/api/*).

use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::sync::LazyLock;

// Admin credentials used to protect /api/*
const ADMIN_USER: &str = "admin";
const ADMIN_PASS: &str = "admin";

static ADMIN_HEADER: LazyLock<String> = LazyLock::new(|| {
    format!(
        "Basic {}",
        STANDARD.encode(format!("{ADMIN_USER}:{ADMIN_PASS}"))
    )
});

/// Rejects admin API requests that don't carry the Basic credentials.
/// Mounted by main.rs via `middleware::from_fn(require_auth)`.
pub async fn require_auth(req: Request, next: Next) -> Response<Body> {
    let authorized = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v == ADMIN_HEADER.as_str());

    if authorized {
        return next.run(req).await;
    }

    tracing::warn!("unauthorized: {} {}", req.method(), req.uri());
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({ "error": "unauthorized" })),
    )
        .into_response()
}
