mod evaluator;

#[derive(serde::Deserialize)]
struct AppConfig {
    redis_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config_ = config::Config::builder()
        .add_source(config::File::with_name("etc/services.yaml").required(false))
        .add_source(config::File::with_name("etc/matchmaker.yaml").required(false))
        .add_source(config::Environment::default())
        .build()?;

    let app_config: AppConfig = config_.try_deserialize()?;

    if let Err(e) = evaluator::run_loop(app_config.redis_url).await {
        tracing::error!("Matchmaker crashed: {:?}", e);
    }

    Ok(())
}
