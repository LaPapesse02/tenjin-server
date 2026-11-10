use axum::{Router, routing::get};
use tracing::{Level, event};

mod config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = config::load_config();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind(format!(
        "{addr}:{port}",
        addr = config.bind_address,
        port = config.port
    ))
    .await
    .unwrap();
    event!(
        Level::INFO,
        "Binding to {addr}:{port}",
        addr = config.bind_address,
        port = config.port
    );
    let _ = axum::serve(listener, app).await;
}

async fn root() -> &'static str {
    "Hello, World!"
}
