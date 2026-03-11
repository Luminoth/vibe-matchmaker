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

#[derive(serde::Deserialize)]
struct AppConfig {
    redis_url: String,
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config_ = config::Config::builder()
        // Common configuration
        .add_source(config::File::with_name("etc/services.yaml").required(false))
        // Service specific configuration
        .add_source(config::File::with_name("etc/api.yaml").required(false))
        // Environment variables as overrides
        .add_source(config::Environment::default())
        .build()?;

    let app_config: AppConfig = config_.try_deserialize()?;

    let state = Arc::new(AppStateInner::new(&app_config.redis_url)?);

    let app = Router::new()
        .route("/matchmaking/start", post(routes::matchmaking::start))
        .route("/matchmaking/cancel", post(routes::matchmaking::cancel))
        .route(
            "/matchmaking/status/:ticket_id",
            get(routes::matchmaking::status),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], app_config.port));
    info!("API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
