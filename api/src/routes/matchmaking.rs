use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use common::{
    models::{
        match_state::TicketStatus,
        ticket::{Player, Ticket},
    },
    redis::keys,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct StartMatchmakingRequest {
    pub match_config_id: String,
    pub players: Vec<Player>,
}

#[derive(Serialize)]
pub struct StartMatchmakingResponse {
    pub ticket_id: Uuid,
}

pub async fn start(
    State(state): State<AppState>,
    Json(payload): Json<StartMatchmakingRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.players.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No players in request".to_string()));
    }

    // TODO: idempotency check (e.g., does the party leader already have a ticket?)

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let ticket = Ticket::new(payload.match_config_id.clone(), payload.players, timestamp);

    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Save ticket data
    let ticket_data_json = serde_json::to_string(&ticket).unwrap();
    let _: () = conn
        .set(keys::ticket_data_key(&ticket.id), ticket_data_json)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Set initial status to Queued
    let initial_status = serde_json::to_string(&TicketStatus::Queued).unwrap();
    let _: () = conn
        .set(keys::ticket_status_key(&ticket.id), initial_status)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Push into the matchmaking queue for this specific config
    // We use a ZSET ordered by join_timestamp_ms so the matchmaker pulls older tickets first
    let _: () = conn
        .zadd(
            keys::queue_key(&payload.match_config_id),
            ticket.id.to_string(),
            ticket.join_timestamp_ms as f64,
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        StatusCode::ACCEPTED,
        Json(StartMatchmakingResponse {
            ticket_id: ticket.id,
        }),
    ))
}

#[derive(Deserialize)]
pub struct CancelMatchmakingRequest {
    pub ticket_id: Uuid,
    pub match_config_id: String,
}

pub async fn cancel(
    State(state): State<AppState>,
    Json(payload): Json<CancelMatchmakingRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Check current status
    let status_json: Option<String> = conn
        .get(keys::ticket_status_key(&payload.ticket_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(s) = status_json {
        let status: TicketStatus = serde_json::from_str(&s).unwrap();
        // Disallow canceling if we are already allocating or completed
        match status {
            TicketStatus::AllocatingServer | TicketStatus::Completed { .. } => {
                return Err((
                    StatusCode::CONFLICT,
                    "Cannot cancel, match already forming".to_string(),
                ));
            }
            _ => {}
        }
    } else {
        return Err((StatusCode::NOT_FOUND, "Ticket not found".to_string()));
    }

    // Remove from queue
    let _: () = conn
        .zrem(
            keys::queue_key(&payload.match_config_id),
            payload.ticket_id.to_string(),
        )
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Clean up ticket data
    let _: () = conn
        .del(&[
            keys::ticket_data_key(&payload.ticket_id),
            keys::ticket_status_key(&payload.ticket_id),
        ])
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: TicketStatus,
    pub connection_ip: Option<String>,
    pub connection_port: Option<u32>,
    pub expected_queue_time: Option<u64>,
}

pub async fn status(
    State(state): State<AppState>,
    Path(ticket_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let status_json: Option<String> = conn
        .get(keys::ticket_status_key(&ticket_id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let status_str =
        status_json.ok_or_else(|| (StatusCode::NOT_FOUND, "Ticket not found".to_string()))?;
    let status: TicketStatus = serde_json::from_str(&status_str).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid status format".to_string(),
        )
    })?;

    let mut connection_ip = None;
    let mut connection_port = None;

    if let TicketStatus::Completed {
        connection_ip: ip,
        connection_port: port,
        ..
    } = &status
    {
        connection_ip = Some(ip.clone());
        connection_port = Some(*port);
    }

    Ok(Json(StatusResponse {
        status,
        connection_ip,
        connection_port,
        expected_queue_time: None, // Could be calculated based on average queue times
    }))
}
