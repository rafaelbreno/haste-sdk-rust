pub mod models {
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

    pub enum HasteEnvironment {
        Production,
        NonProduction,
    }

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
}
