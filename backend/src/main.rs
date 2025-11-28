mod clients;
mod aggregator;
mod cache;
mod persistence;
mod api;
mod health;
mod types;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting GoQuant Oracle Backend");
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/", get(|| async { "oracle backend" }));
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    info!("Listening on {}", addr);
    hyper::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
