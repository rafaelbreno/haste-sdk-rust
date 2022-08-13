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
