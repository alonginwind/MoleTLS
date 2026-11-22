mod assets;
mod auth;
mod proxy;
mod services;
mod status;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{Response, StatusCode},
    middleware::from_fn,
    response::IntoResponse,
    routing::{delete, get},
    Router,
};
use proxy::AppState;
use status::LISTEN_PORT;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::new();

    let api_routes = Router::new()
        .route("/api/status", get(status::status))
        .route("/api/services", get(services::list_services).post(services::add_service))
        .route("/api/services/:name", delete(services::delete_service))
        .layer(from_fn(auth::require_auth));

    let app = Router::new()
        .merge(api_routes)
        .fallback(serve_or_proxy)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", LISTEN_PORT))
        .await
        .unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    tracing::info!("Open http://localhost:{LISTEN_PORT}/");
    axum::serve(listener, app).await.unwrap();
}

/// Fallback dispatcher: embedded frontend for localhost, domain-matched
/// reverse proxy for everything else.
async fn serve_or_proxy(State(state): State<AppState>, req: Request) -> Response<Body> {
    let req_host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("")
        .to_string();

    let path = req.uri().path().trim_start_matches('/');

    // Serve frontend assets only for localhost/127.0.0.1
    if req_host == "localhost" || req_host == "127.0.0.1" {
        if !path.is_empty() && assets::asset_exists(path) {
            return assets::serve_asset(path);
        }
        if path.is_empty() && assets::asset_exists("index.html") {
            return assets::serve_asset("index.html");
        }
    }

    // Match proxy by domain
    if let Some(resp) = proxy::forward(req, &state, &req_host).await {
        return resp;
    }

    tracing::warn!("no proxy matched for host: {}", req_host);
    StatusCode::NOT_FOUND.into_response()
}
