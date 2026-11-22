// Frontend files embedded at compile time by rust-embed.

use axum::{
    body::Body,
    http::{header, Response},
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "target/frontend/"]
pub struct Assets;

/// Returns true when the embedded bundle contains the given file.
pub fn asset_exists(path: &str) -> bool {
    Assets::get(path).is_some()
}

pub fn serve_asset(path: &str) -> Response<Body> {
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
