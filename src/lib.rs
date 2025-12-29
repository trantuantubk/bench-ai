pub mod handlers;
pub mod models;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/models", post(handlers::models::create_model));
    axum::serve(listener, app)
        .await
        .map_err(std::io::Error::other)
}
