// Domain-based reverse proxy: shared config state and request forwarding.

use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock, RwLock};

// Reuse a single client for connection pooling
static HTTP_CLIENT: LazyLock<Client<hyper_util::client::legacy::connect::HttpConnector, Body>> =
    LazyLock::new(|| Client::builder(TokioExecutor::new()).build_http());

#[derive(Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub name: String,
    pub domain: String,
    pub target_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub proxies: Arc<RwLock<Vec<ProxyConfig>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            proxies: Arc::new(RwLock::new(Vec::new())),
        }
    }
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

/// Forward the request to the proxy entry matching `req_host`.
/// Returns `None` when no domain matches so the caller can fall through.
pub async fn forward(req: Request, state: &AppState, req_host: &str) -> Option<Response<Body>> {
    let configs = state.proxies.read().unwrap().clone();
    let config = find_proxy_by_domain(&configs, req_host)?;

    tracing::info!("{} {} -> {}", req_host, req.uri().path(), config.target_url);
    Some(proxy_to(req, config).await.into_response())
}
