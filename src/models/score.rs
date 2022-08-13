use super::leader::Leader;

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
