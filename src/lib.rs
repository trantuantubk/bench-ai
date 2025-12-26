pub mod handlers;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/health", get(handlers::health::health_check));
    axum::serve(listener, app)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

