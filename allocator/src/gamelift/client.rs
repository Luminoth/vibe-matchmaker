use aws_sdk_gamelift::Client;
use common::MatchmakerError;
use tracing::info;

pub struct GameLiftManager {
    client: Client,
    fleet_id: String,
}

impl GameLiftManager {
    pub async fn new(fleet_id: String) -> Result<Self, MatchmakerError> {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = Client::new(&config);

        Ok(Self { client, fleet_id })
    }

    pub async fn allocate_server(
        &self,
        match_id: &str,
    ) -> Result<(String, u32, Option<String>), MatchmakerError> {
        info!("Allocating GameLift server for match: {}", match_id);

        let response = self.client.create_game_session()
            .fleet_id(&self.fleet_id)
            .maximum_player_session_count(10) // In reality this would come from the match size or config
            .game_properties(aws_sdk_gamelift::types::GameProperty::builder()
                .key("match_id")
                .value(match_id)
                .build())
            .send()
            .await
            .map_err(|e| {
                tracing::error!("GameLift CreateGameSession error: {:?}", e);
                MatchmakerError::Internal
            })?;

        let game_session = response.game_session().ok_or_else(|| {
            tracing::error!("GameLift response missing GameSession");
            MatchmakerError::Internal
        })?;

        // IP Address and Port are guaranteed to be present for active sessions, but GameLift initially puts them in ACTIVATING state.
        // For a real production system we might need to place matches into `AllocatingServer` queue and poll DescribeGameSessions,
        // or rely on Amazon EventBridge -> SQS -> Lambda to notify us when the session is ACTIVE with IP/Port.
        // Assuming instantaneous or pre-allocated for this simplified flow.
        let ip = game_session.ip_address().unwrap_or("0.0.0.0").to_string();
        let port = game_session.port().unwrap_or(0) as u32;
        let dns = game_session.dns_name().map(|d| d.to_string());

        Ok((ip, port, dns))
    }
}
