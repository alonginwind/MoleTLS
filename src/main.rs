use axum::{
    body::Body,
    extract::Request,
    http::{header, Response, StatusCode},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use rust_embed::Embed;
use serde::Serialize;
use std::sync::LazyLock;

const BACKEND: &str = "httpbin.org";

// Embed frontend files at compile time
#[derive(Embed)]
#[folder = "target/frontend/"]
struct Assets;

// Reuse a single client for connection pooling
static HTTP_CLIENT: LazyLock<Client<hyper_util::client::legacy::connect::HttpConnector, Body>> =
    LazyLock::new(|| Client::builder(TokioExecutor::new()).build_http());

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/status", get(status))
        .fallback(serve_or_proxy);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7999").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    tracing::info!("Open http://localhost:7999/");
    axum::serve(listener, app).await.unwrap();
}

// Serve embedded frontend files, or proxy to backend
async fn serve_or_proxy(req: Request) -> Response<Body> {
    let path = req.uri().path().trim_start_matches('/');

    // Serve exact match from embedded assets (index.html, *.js, *.wasm, etc.)
    if !path.is_empty() && Assets::get(path).is_some() {
        return serve_asset(path);
    }

    // Root path serves index.html
    if path.is_empty() {
        if Assets::get("index.html").is_some() {
            return serve_asset("index.html");
        }
    }

    // Everything else: proxy to backend
    proxy(req).await.into_response()
}

// Serve a file from embedded assets
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

async fn proxy(req: Request) -> impl IntoResponse {
    let method = req.method().clone();
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");

    let uri = if query.is_empty() {
        format!("http://{}{}", BACKEND, path)
    } else {
        format!("http://{}{}?{}", BACKEND, path, query)
    };

    tracing::info!("{} {} -> {}", method, path, BACKEND);

    let (parts, body) = req.into_parts();

    let mut builder = hyper::Request::builder().method(method).uri(&uri);
    for (k, v) in &parts.headers {
        if k != "host" {
            builder = builder.header(k, v);
        }
    }
    builder = builder.header("Host", BACKEND);

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

#[derive(Serialize)]
struct Status {
    status: &'static str,
    backend: &'static str,
    port: u16,
}

async fn status() -> Json<Status> {
    Json(Status {
        status: "running",
        backend: BACKEND,
        port: 7999,
    })
}
