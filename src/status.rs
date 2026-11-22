// Health/status endpoint (/api/status).

use axum::response::Json;
use serde::Serialize;

pub const LISTEN_PORT: u16 = 7999;

#[derive(Serialize)]
pub struct Status {
    status: &'static str,
    backend: &'static str,
    port: u16,
}

pub async fn status() -> Json<Status> {
    Json(Status {
        status: "running",
        backend: "httpbin.org",
        port: LISTEN_PORT,
    })
}
