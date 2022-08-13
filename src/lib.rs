pub mod models {
    pub mod arcade;
    pub mod haste_environment;
    pub mod leader;
    pub mod leaderboard;
    pub mod game;
    pub mod payout;
    pub mod play;
    pub mod player;
    pub mod score;
    pub mod token;

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
