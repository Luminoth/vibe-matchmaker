use common::models::{config::MatchConfig, ticket::Ticket};

pub fn evaluate_tickets(
    config: &MatchConfig,
    tickets: &[Ticket],
    current_time_ms: u64,
) -> Option<Vec<Ticket>> {
    tracing::debug!("Evaluating tickets at time: {}", current_time_ms);
    let mut current_group = Vec::new();
    let mut current_players = 0;

    // Simplistic grouping: just try to greedily fill a match up to max_players
    for ticket in tickets {
        let ticket_size = ticket.size() as u32;

        if current_players + ticket_size <= config.max_players {
            let mut can_add = true;

            // Enforce team sizes
            if ticket_size < config.min_team_size || ticket_size > config.max_team_size {
                can_add = false;
            }

            // TODO: Skill evaluation
            // - Calculate expansion gap for this ticket based on (current_time_ms - ticket.join_timestamp_ms)
            // - Ensure ticket overlaps with the current group's average rating

            // TODO: Ping evaluation
            // - Find intersection of acceptable ping regions

            if can_add {
                current_group.push(ticket.clone());
                current_players += ticket_size;
            }
        }

        if current_players >= config.min_players {
            return Some(current_group);
        }
    }

    None
}
