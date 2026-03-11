use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing::info;

mod routes;
mod state;

use state::AppStateInner;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let state = Arc::new(AppStateInner::new(&redis_url)?);

    let app = Router::new()
        .route("/matchmaking/start", post(routes::matchmaking::start))
        .route("/matchmaking/cancel", post(routes::matchmaking::cancel))
        .route(
            "/matchmaking/status/:ticket_id",
            get(routes::matchmaking::status),
        )
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
