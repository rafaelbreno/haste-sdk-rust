use super::leaderboard::*;

pub struct Game {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub developer_info: Option<String>,
    pub tag_line: Option<String>,
    pub url: String,
    pub leaderboards: Vec<Leaderboard>, 
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
        leaderboards: Vec<Leaderboard>, 
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
