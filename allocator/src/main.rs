pub mod gamelift;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let fleet_id = std::env::var("GAMELIFT_FLEET_ID").unwrap_or_else(|_| "fleet-dummy".to_string());

    let manager = gamelift::client::GameLiftManager::new(fleet_id).await?;

    if let Err(e) = gamelift::run_loop(redis_url, manager).await {
        tracing::error!("Allocator crashed: {:?}", e);
    }

    Ok(())
}
