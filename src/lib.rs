pub mod models {
    use chrono::{DateTime, Utc};

    // Arcade
    #[derive(PartialEq,Debug)]
    pub struct Arcade {
        pub id: String,
        pub name: String,
        pub description: String,
    }

    pub fn new_arcade(id: String) -> Arcade {
       Arcade {
           id, 
           name: String::from(""), 
           description: String::from(""),
       } 
    }

    // HasteEnvironment
    pub enum HasteEnvironment {
        Production,
        NonProduction,
    }

    // Leader
    pub struct Leader {
        pub player_id: String,
        pub score: u64,
        pub name: Option<String>,
        pub avatar: Option<String>,
    }

    pub fn new_leader(
            player_id: String,
            score: u64,
            name: Option<String>, 
            avatar: Option<String>,
        ) -> Leader {
        Leader {
            player_id,
            score,
            name,
            avatar,
        }
    }

    // Leaderboard
    pub struct Leaderboard {
        pub id: String,
        pub name: String,
        pub cost: u64,
        pub formatted_cost_string: String,
        pub currency: String,
        pub leaders: Option<[String; 0]>,
        pub formatted_name: String,
    }

    pub fn new_leaderboard(
            id: String,
            name: String,
            cost: u64,
            formatted_cost_string: String,
            currency: String,
            leaders: Option<[String; 0]>, 
            formatted_name: String,
        ) -> Leaderboard {
        Leaderboard {
            id,
            name,
            cost,
            formatted_cost_string,
            currency,
            leaders,
            formatted_name,
        }
    }

    // Game
    pub struct Game {
        pub id: String,
        pub name: String,
        pub description: String,
        pub category: String,
        pub developer_info: Option<String>,
        pub tag_line: Option<String>,
        pub url: String,
        pub leaderboards: Option<[Leaderboard; 0]>, 
        pub player_id: String,
    }

    pub fn new_game(
            id: String,
            name: String,
            description: String,
            category: String,
            developer_info: Option<String>,
            tag_line: Option<String>,
            url: String,
            leaderboards: Option<[Leaderboard; 0]>, 
            player_id: String,
        ) -> Game {
        Game {
            id,
            name,
            description,
            category,
            developer_info,
            tag_line,
            url,
            leaderboards,
            player_id,
        }
    }

    // Payout
    pub struct PayoutDetail {
        pub user_id: String,
        pub created_at: DateTime<Utc>,
        pub payee_handle: String,
        pub payer_handle: String,
        pub payment_amount: f64,
    }

    pub fn new_payout_detail(
        user_id: String,
        created_at: DateTime<Utc>,
        payee_handle: String,
        payer_handle: String,
        payment_amount: f64,
        ) -> PayoutDetail {
        PayoutDetail {
            user_id,
            created_at,
            payee_handle,
            payer_handle,
            payment_amount,
        }
    }
    
    pub struct PayoutEvent {
        pub event_id: String,
        pub details: PayoutDetail,
    }

    pub fn new_payout_event (
        event_id: String,
        details: PayoutDetail,
    ) -> PayoutEvent {
        PayoutEvent {
            event_id,
            details,
        }
    }

    pub struct Payout {
        pub starting_after: String,
        pub ending_before: String,
        pub events: Option<[PayoutEvent; 0]>,
    }

    pub fn new_payout (
        starting_after: String,
        ending_before: String,
        events: Option<[PayoutEvent; 0]>,
    ) -> Payout {
        Payout {
            starting_after,
            ending_before,
            events,
        }
    }

    // Play
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
