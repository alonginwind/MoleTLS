use axum::{extract::Request, routing::any, Router};
use hyper_util::{client::legacy::Client, rt::TokioExecutor};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/*path", any(proxy));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7999").await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn proxy(req: Request) -> impl axum::response::IntoResponse {
    let uri = format!("http://httpbin.org{}", req.uri().path());
    let (parts, body) = req.into_parts();

    let mut builder = hyper::Request::builder().method(parts.method).uri(&uri);
    for (k, v) in &parts.headers {
        builder = builder.header(k, v);
    }

    let client = Client::builder(TokioExecutor::new()).build_http::<axum::body::Body>();
    client.request(builder.body(body).unwrap()).await.unwrap()
}
