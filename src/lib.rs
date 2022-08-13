pub mod models {
    pub mod arcade;
    pub mod haste_environment;
    pub mod leader;
    pub mod leaderboard;
    pub mod game;
    pub mod payout;
    pub mod play;
    pub mod player;

    use leader::Leader;

    // Score 
    pub struct Score {
        pub id: String,
        pub score: u64,
        pub leaders: [Leader; 0],
        pub is_winner: bool,
        pub leader_rank: u64,
    }

    pub fn new_score(
        id: String,
        score: u64,
        leaders: [Leader; 0],
        is_winner: bool,
        leader_rank: u64,
    ) -> Score {
        Score {
            id,
            score,
            leaders,
            is_winner,
            leader_rank,
        }
    }

    pub struct CreateScore {
        pub play_id: String,
        pub score: u64,
        pub leaderboard_id: String,
    }

    pub fn new_create_score(
        play_id: String,
        score: u64,
        leaderboard_id: String,
    ) -> CreateScore {
        CreateScore {
            play_id,
            score,
            leaderboard_id,
        }
    }

    use haste_environment::HasteEnvironment;

    // TokenRequest
    pub struct TokenRequest {
        pub client_id: String,
        pub client_secret: String,
        pub environment: HasteEnvironment,
    }

    pub fn new_token_request(
        client_id: String,
        client_secret: String,
        environment: HasteEnvironment,
    ) -> TokenRequest {
        TokenRequest {
            client_id,
            client_secret,
            environment,
        }
    }

    // TokenResponse
    pub struct TokenResponse {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: Option<String>,
        pub scope: Option<String>,
        pub arcade_id: String,
        pub game_id: String,
    }

    pub fn new_token_response(
        access_token: String,
        token_type: String,
        expires_in: Option<String>,
        scope: Option<String>,
        arcade_id: String,
        game_id: String,
    ) -> TokenResponse {
        TokenResponse {
            access_token,
            token_type,
            expires_in,
            scope,
            arcade_id,
            game_id,
        }
    }

    // TopScore
    pub struct TopScore {
        pub user_id: String,
        pub value: u64,
    }

    pub fn new_top_score(
        user_id: String,
        value: u64,
    ) -> TopScore {
        TopScore {
            user_id,
            value,
        }
    }
}
