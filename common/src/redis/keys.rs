use uuid::Uuid;

/// Returns the key for the list/zset of tickets actively waiting in the queue for a given config.
pub fn queue_key(config_id: &str) -> String {
    format!("matchmaker:queue:{}", config_id)
}

/// Returns the key storing the JSON representation of an individual ticket's request data.
pub fn ticket_data_key(ticket_id: &Uuid) -> String {
    format!("matchmaker:ticket:{}:data", ticket_id)
}

/// Returns the key storing the JSON representation of an individual ticket's current status.
pub fn ticket_status_key(ticket_id: &Uuid) -> String {
    format!("matchmaker:ticket:{}:status", ticket_id)
}

/// Returns the key for the pool of formed matches waiting for GameLift allocation.
pub fn allocation_queue_key() -> String {
    "matchmaker:allocation:queue".to_string()
}

/// Returns the key storing the JSON representation of an individual match.
pub fn match_data_key(match_id: &str) -> String {
    format!("matchmaker:match:{}", match_id)
}

/// Returns the key storing the JSON representation of a given MatchConfig.
pub fn config_key(config_id: &str) -> String {
    format!("matchmaker:config:{}", config_id)
}
