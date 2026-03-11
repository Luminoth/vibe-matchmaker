use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TicketStatus {
    Queued,
    ReadyCheck,
    AllocatingServer,
    Completed {
        match_id: String,
        connection_ip: String,
        connection_port: u32,
        connection_dns_name: Option<String>,
    },
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: String,
    pub match_config_id: String,
    pub tickets: Vec<crate::models::ticket::Ticket>,
    pub status: TicketStatus,

    // Ready Check State
    // Ticket ID -> has accepted
    pub ready_check_status: HashMap<uuid::Uuid, bool>,
    pub ready_check_started_at_ms: Option<u64>,

    // Server Info
    pub connection_ip: Option<String>,
    pub connection_port: Option<u32>,
    pub connection_dns_name: Option<String>,
}

impl Match {
    pub fn new(
        id: String,
        match_config_id: String,
        tickets: Vec<crate::models::ticket::Ticket>,
    ) -> Self {
        Self {
            id,
            match_config_id,
            tickets,
            status: TicketStatus::Queued,
            ready_check_status: HashMap::new(),
            ready_check_started_at_ms: None,
            connection_ip: None,
            connection_port: None,
            connection_dns_name: None,
        }
    }
}
