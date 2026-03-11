use aws_sdk_gamelift::Client;
use common::MatchmakerError;
use tracing::info;

#[allow(dead_code)]
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

        /*
        // REAL IMPLEMENTATION WOULD BE:

        let response = self.client.create_game_session()
            .fleet_id(&self.fleet_id)
            .maximum_player_session_count(10)
            .game_properties(aws_sdk_gamelift::types::GameProperty::builder()
                .key("match_id")
                .value(match_id)
                .build())
            .send()
            .await
            .map_err(|e| MatchmakerError::Internal)?; // properly map error

        let game_session = response.game_session().unwrap();
        let ip = game_session.ip_address().unwrap().to_string();
        let port = game_session.port().unwrap() as u32;
        let dns = game_session.dns_name().map(|d| d.to_string());

        return Ok((ip, port, dns));
        */

        // STUB IMPLEMENTATION for local testing without AWS creds
        Ok(("127.0.0.1".to_string(), 7777, None))
    }
}
