use common::MatchmakerError;
use redis::Client;
use std::sync::Arc;

pub struct AppStateInner {
    pub redis_client: Client,
}

pub type AppState = Arc<AppStateInner>;

impl AppStateInner {
    pub fn new(redis_url: &str) -> Result<Self, MatchmakerError> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            redis_client: client,
        })
    }
}
