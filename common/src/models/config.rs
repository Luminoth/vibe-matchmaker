use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MatchConfig {
    pub id: String,

    // Player bounds
    pub min_players: u32,
    pub max_players: u32,

    // Team bounds
    pub max_teams: u32,
    pub min_team_size: u32,
    pub max_team_size: u32,

    // Skill constraints
    pub is_skill_based: bool,
    pub skill_gap_expansion_start_seconds: u64,
    pub skill_gap_expansion_rate: f64,

    // Time constraints
    pub time_in_queue_weight: f64,
    pub max_time_in_queue_seconds: u64, // 0 means infinite

    // Network constraints
    pub ping_gap_expansion_start_seconds: u64,
    pub ping_gap_expansion_rate: f64,

    // Match rules
    pub allow_crossplay: bool,
    pub game_level: String,

    // Backfill & late join
    pub supports_backfill: bool,
    pub backfill_weight: f64,
    pub supports_late_join: bool,

    // Ready Check
    pub require_ready_check: bool,
    pub ready_check_timeout_seconds: u64,
}
