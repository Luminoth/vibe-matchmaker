use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatchmakerError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration not found: {0}")]
    ConfigNotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Internal server error")]
    Internal,
}
