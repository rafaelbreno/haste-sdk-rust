use super::leaderboard::*;
use chrono::{DateTime, Utc};
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
