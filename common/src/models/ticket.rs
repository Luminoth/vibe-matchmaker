use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub skill_rating: Option<f64>,
    pub ping_to_regions: HashMap<String, u32>,
    pub block_list: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub match_config_id: String,

    pub players: Vec<Player>,
    pub is_party: bool,

    // The timestamp when this ticket entered the queue
    pub join_timestamp_ms: u64,
}

impl Ticket {
    pub fn new(match_config_id: String, players: Vec<Player>, join_timestamp_ms: u64) -> Self {
        let is_party = players.len() > 1;
        Self {
            id: Uuid::new_v4(),
            match_config_id,
            players,
            is_party,
            join_timestamp_ms,
        }
    }

    pub fn size(&self) -> usize {
        self.players.len()
    }
}
