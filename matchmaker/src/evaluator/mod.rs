use common::MatchmakerError;

use std::time::Duration;
use tracing::info;

pub mod rules;

pub async fn run_loop(redis_url: String) -> Result<(), MatchmakerError> {
    info!("Starting matchmaking evaluator using {}", redis_url);

    let client = redis::Client::open(redis_url)?;
    let mut _conn = client.get_multiplexed_async_connection().await?;

    // In a real system, you'd want to track ALL active configs
    // For this example, we'll just poll a known config or have a set of defined configs.
    // E.g., loading config from DB on startup:

    // We would fetch distinct `matchmaker:queue:*` keys and process them.
    // For simplicity, we'll just fake scanning queues.

    let dummy_config = common::models::match_config::MatchConfig::default();

    loop {
        // Here we would:
        // 1. Get all active match_config_ids
        // 2. Fetch the queue ZRANGEBYSCORE for each config
        // 3. Apply grouping rules to the tickets
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let _ = rules::evaluate_tickets(&dummy_config, &[], current_time);

        // 4. If a match is formed, transition statuses and push to allocator queue

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
