pub mod models {
    use chrono::{DateTime, Utc};
    pub mod arcade;
    pub mod haste_environment;
    pub mod leader;
    pub mod leaderboard;
    pub mod game;
    pub mod payout;

    // Play
    use leaderboard::*;
    pub struct Play {
        pub id: String,
        pub game_id: String,
        pub player_id: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub deleted_at: Option<DateTime<Utc>>,
        // TODO:
        //  - Check if there's a need for `Option`, or could use the bool
        //      directly. Because in the "original" SDK (TS) the field is optional.
        pub deleted: Option<bool>,
        pub leaderboard: Leaderboard,
        pub cost: f64,
    }

    pub fn new_play (
        id: String,
        game_id: String,
        player_id: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
        deleted: Option<bool>,
        leaderboard: Leaderboard,
        cost: f64,
    ) -> Play {
        Play {
            id,
            game_id,
            player_id,
            created_at,
            updated_at,
            deleted_at,
            deleted,
            leaderboard,
            cost,
        }
    }

    pub struct PlayTransaction {
        pub id: String,
        pub status: String,
        pub tx: String,
    }

    pub fn new_play_transaction (
        id: String,
        status: String,
        tx: String,
    ) -> PlayTransaction {
        PlayTransaction {
            id,
            status,
            tx,
        }
    }

    pub struct CreatePlay {
        pub player_id: String,
        pub leaderboard_id: String,
    }

    pub fn new_create_play (
        player_id: String,
        leaderboard_id: String,
    ) -> CreatePlay {
        CreatePlay { 
            player_id, 
            leaderboard_id,
        }
    }

    // Player
    pub struct Player {
        pub id: String,
    }

    pub fn new_player (id: String) ->Player {
        Player { id }
    }

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
