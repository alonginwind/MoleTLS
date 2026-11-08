use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::{header, Response, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get},
    Router,
};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock, RwLock};

// Embed frontend files at compile time
#[derive(Embed)]
#[folder = "target/frontend/"]
struct Assets;

// Reuse a single client for connection pooling
static HTTP_CLIENT: LazyLock<Client<hyper_util::client::legacy::connect::HttpConnector, Body>> =
    LazyLock::new(|| Client::builder(TokioExecutor::new()).build_http());

#[derive(Clone, Serialize, Deserialize)]
struct ProxyConfig {
    name: String,
    domain: String,
    target_url: String,
}

#[derive(Clone)]
struct AppState {
    proxies: Arc<RwLock<Vec<ProxyConfig>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        proxies: Arc::new(RwLock::new(Vec::new())),
    };

    let app = Router::new()
        .route("/api/status", get(status))
        .route("/api/services", get(list_services).post(add_service))
        .route("/api/services/:name", delete(delete_service))
        .fallback(serve_or_proxy)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7999").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    tracing::info!("Open http://localhost:7999/");
    axum::serve(listener, app).await.unwrap();
}

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
        if !path.is_empty() && Assets::get(path).is_some() {
            return serve_asset(path);
        }
        if path.is_empty() && Assets::get("index.html").is_some() {
            return serve_asset("index.html");
        }
    }

    // Match proxy by domain
    let configs = state.proxies.read().unwrap().clone();
    if let Some(config) = find_proxy_by_domain(&configs, &req_host) {
        tracing::info!("{} {} -> {}", req_host, req.uri().path(), config.target_url);
        return proxy_to(req, &config).await.into_response();
    }

    tracing::warn!("no proxy matched for host: {}", req_host);
    StatusCode::NOT_FOUND.into_response()
}

fn find_proxy_by_domain<'a>(configs: &'a [ProxyConfig], host: &str) -> Option<&'a ProxyConfig> {
    configs.iter().find(|c| c.domain == host)
}

async fn proxy_to(req: Request, config: &ProxyConfig) -> impl IntoResponse {
    let method = req.method().clone();
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");

    let target_base = config.target_url.trim_end_matches('/');
    let uri = if query.is_empty() {
        format!("{}{}", target_base, path)
    } else {
        format!("{}{}?{}", target_base, path, query)
    };

    tracing::info!("{} {} -> {}", method, path, uri);

    let (parts, body) = req.into_parts();
    let target_host = config.target_url.split("://").nth(1).unwrap_or(&config.target_url);
    let target_host = target_host.split('/').next().unwrap_or(target_host);

    let mut builder = hyper::Request::builder().method(method).uri(&uri);
    for (k, v) in &parts.headers {
        if k != "host" {
            builder = builder.header(k, v);
        }
    }
    builder = builder.header("Host", target_host);

    let outbound = match builder.body(body) {
        Ok(req) => req,
        Err(e) => {
            tracing::error!("Failed to build request: {}", e);
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    match HTTP_CLIENT.request(outbound).await {
        Ok(resp) => resp.into_response(),
        Err(e) => {
            tracing::error!("Backend request failed: {}", e);
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}

// --- API handlers ---

async fn list_services(State(state): State<AppState>) -> Json<Vec<ProxyConfig>> {
    Json(state.proxies.read().unwrap().clone())
}

async fn add_service(State(state): State<AppState>, Json(config): Json<ProxyConfig>) -> Json<ProxyConfig> {
    state.proxies.write().unwrap().push(config.clone());
    Json(config)
}

async fn delete_service(State(state): State<AppState>, Path(name): Path<String>) -> StatusCode {
    let mut services = state.proxies.write().unwrap();
    if let Some(pos) = services.iter().position(|s| s.name == name) {
        services.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// --- Status ---

#[derive(Serialize)]
struct Status {
    status: &'static str,
    backend: &'static str,
    port: u16,
}

async fn status() -> Json<Status> {
    Json(Status {
        status: "running",
        backend: "httpbin.org",
        port: 7999,
    })
}

// --- Static assets ---

fn serve_asset(path: &str) -> Response<Body> {
    let asset = Assets::get(path).unwrap();
    let mime = mime_from_path(path);
    let body = Body::from(asset.data.into_owned());
    Response::builder()
        .header(header::CONTENT_TYPE, mime)
        .body(body)
        .unwrap()
}

fn mime_from_path(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html",
        Some("js") => "application/javascript",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}
