use chrono::{Date, Utc};

use crate::models::haste_environment::HasteEnvironment;

pub struct HasteConfiguration {
    pub api_version: Option<String>,
    pub host_protocol: String,
    pub host: String,
    pub port: Option<u16>,
    pub arcade_id: Option<String>,
    pub game_id: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub access_token: String,
    pub player_id: Option<String>,
    pub environment: HasteEnvironment,
    pub token_expiration: Date<Utc>,
}

pub fn new_haste_configuration(
    api_version: Option<String>,
    host_protocol: String,
    host: String,
    port: Option<u16>,
    arcade_id: Option<String>,
    game_id: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    access_token: String,
    player_id: Option<String>,
    environment: HasteEnvironment,
    token_expiration: Date<Utc>,
) -> HasteConfiguration {
    HasteConfiguration { 
        api_version,
        host_protocol,
        host,
        port,
        arcade_id,
        game_id,
        client_id,
        client_secret,
        access_token,
        player_id,
        environment,
        token_expiration,
    }
}
