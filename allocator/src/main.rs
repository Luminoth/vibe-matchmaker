pub mod gamelift;

#[derive(serde::Deserialize)]
struct AppConfig {
    redis_url: String,
    fleet_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config_ = config::Config::builder()
        .add_source(config::File::with_name("etc/services.yaml").required(false))
        .add_source(config::File::with_name("etc/allocator.yaml").required(false))
        .add_source(config::Environment::default())
        .build()?;

    let app_config: AppConfig = config_.try_deserialize()?;

    let manager = gamelift::client::GameLiftManager::new(app_config.fleet_id).await?;

    if let Err(e) = gamelift::run_loop(app_config.redis_url, manager).await {
        tracing::error!("Allocator crashed: {:?}", e);
    }

    Ok(())
}
