use crate::{models::{game::Game, leaderboard::{Leaderboard, new_leaderboard}}, server::haste_configuration::HasteConfiguration};

pub struct GameResource {
    pub details: Game,
}

pub fn new_game_resource(_configuration: HasteConfiguration, mut details: Game) -> GameResource {
    let leaderboards = &details.leaderboards;
    let mut new_leaderboards: Vec<Leaderboard> = Vec::new();

    for l in leaderboards {
        new_leaderboards.push(
            new_leaderboard(
                l.id.to_string(),
                l.name.to_string(),
                l.cost,
                match l.formatted_cost_string.len() {
                    0 => l.formatted_cost_string.to_string(),
                    _ => format!("{} - {} {}", l.name, l.cost, l.currency),
                },
                l.currency.to_string(),
                l.leaders.to_owned(),
                l.formatted_name.to_string(),
            )
        );
    }

    details.leaderboards = new_leaderboards;

    GameResource {
        details,
    }
}
