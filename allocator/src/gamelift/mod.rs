use crate::gamelift::client::GameLiftManager;
use common::{models::match_state::Match, redis::keys, MatchmakerError};
use redis::AsyncCommands;
use tracing::{info, warn};

pub mod client;

pub async fn run_loop(redis_url: String, manager: GameLiftManager) -> Result<(), MatchmakerError> {
    info!("Starting server allocator engine using {}", redis_url);

    let client = redis::Client::open(redis_url)?;
    let mut conn = client.get_multiplexed_tokio_connection().await?;

    let queue_key = keys::allocation_queue_key();

    loop {
        // Block pop from the allocation queue
        let result: Option<(String, String)> = conn.blpop(&queue_key, 5.0).await.unwrap_or(None);

        if let Some((_, match_json)) = result {
            if let Ok(mut match_data) = serde_json::from_str::<Match>(&match_json) {
                match manager.allocate_server(&match_data.id).await {
                    Ok((ip, port, dns)) => {
                        info!(
                            "Allocated server for match {}: {}:{}",
                            match_data.id, ip, port
                        );

                        // Update the match state
                        let completed_status =
                            common::models::match_state::TicketStatus::Completed {
                                match_id: match_data.id.clone(),
                                connection_ip: ip.clone(),
                                connection_port: port,
                                connection_dns_name: dns.clone(),
                            };
                        match_data.status = completed_status;
                        match_data.connection_ip = Some(ip.clone());
                        match_data.connection_port = Some(port);
                        match_data.connection_dns_name = dns.clone();

                        // Save updated match object back to Redis
                        let _: () = conn
                            .set(
                                keys::match_data_key(&match_data.id),
                                serde_json::to_string(&match_data).unwrap(),
                            )
                            .await
                            .unwrap();

                        // Also update each individual ticket's status so the players polling can see it
                        let complete_status = serde_json::to_string(
                            &common::models::match_state::TicketStatus::Completed {
                                match_id: match_data.id.clone(),
                                connection_ip: ip,
                                connection_port: port,
                                connection_dns_name: dns,
                            },
                        )
                        .unwrap();

                        for ticket in match_data.tickets {
                            let _: () = conn
                                .set(keys::ticket_status_key(&ticket.id), &complete_status)
                                .await
                                .unwrap();
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Failed to allocate server for match {}: {:?}",
                            match_data.id, e
                        );
                        // Requeue or mark failed
                    }
                }
            }
        }
    }
}
