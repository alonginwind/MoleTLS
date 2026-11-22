// Admin API handlers for managing reverse proxy entries (/api/services).

use crate::proxy::{AppState, ProxyConfig};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};

pub async fn list_services(State(state): State<AppState>) -> Json<Vec<ProxyConfig>> {
    Json(state.proxies.read().unwrap().clone())
}

pub async fn add_service(State(state): State<AppState>, Json(config): Json<ProxyConfig>) -> Json<ProxyConfig> {
    state.proxies.write().unwrap().push(config.clone());
    Json(config)
}

pub async fn delete_service(State(state): State<AppState>, Path(name): Path<String>) -> StatusCode {
    let mut services = state.proxies.write().unwrap();
    if let Some(pos) = services.iter().position(|s| s.name == name) {
        services.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
