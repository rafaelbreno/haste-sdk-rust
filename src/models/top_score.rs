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
