use axum::{Router, routing::get};

const DEFAULT_BIND_ADDR: &str = "0.0.0.0";
const DEFAULT_PORT     : &str = "9797";


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind(
        format!("{DEFAULT_BIND_ADDR}:{DEFAULT_PORT}")
    ).await.unwrap();
    let _ = axum::serve(listener, app).await;
}


async fn root() -> &'static str {
    "Hello, World!"
}
