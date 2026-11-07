use axum::{extract::Request, http::StatusCode, response::IntoResponse, routing::any, Router};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use std::sync::LazyLock;

const BACKEND: &str = "httpbin.org";

// Reuse a single client for connection pooling
static HTTP_CLIENT: LazyLock<Client<hyper_util::client::legacy::connect::HttpConnector, axum::body::Body>> =
    LazyLock::new(|| Client::builder(TokioExecutor::new()).build_http());

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/*path", any(proxy));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7999").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn proxy(req: Request) -> impl IntoResponse {
    let method = req.method().clone();
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");

    // Build target URI with query string
    let uri = if query.is_empty() {
        format!("http://{}{}", BACKEND, path)
    } else {
        format!("http://{}{}?{}", BACKEND, path, query)
    };

    tracing::info!("{} {} -> {}", method, path, BACKEND);

    let (parts, body) = req.into_parts();

    // Build forwarded request
    let mut builder = hyper::Request::builder().method(method).uri(&uri);

    // Copy headers, but override Host
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

    // Send request to backend
    match HTTP_CLIENT.request(outbound).await {
        Ok(resp) => resp.into_response(),
        Err(e) => {
            tracing::error!("Backend request failed: {}", e);
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}
