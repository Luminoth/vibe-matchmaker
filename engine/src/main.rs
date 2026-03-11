mod evaluator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    if let Err(e) = evaluator::run_loop(redis_url).await {
        tracing::error!("Engine crashed: {:?}", e);
    }

    Ok(())
}
