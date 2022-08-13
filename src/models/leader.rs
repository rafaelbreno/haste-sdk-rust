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
