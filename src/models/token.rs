use super::haste_environment::HasteEnvironment;

pub struct TokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub environment: HasteEnvironment,
}

pub fn new_token_request(
    client_id: String,
    client_secret: String,
    environment: HasteEnvironment,
) -> TokenRequest {
    TokenRequest {
        client_id,
        client_secret,
        environment,
    }
}

pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<String>,
    pub scope: Option<String>,
    pub arcade_id: String,
    pub game_id: String,
}

pub fn new_token_response(
    access_token: String,
    token_type: String,
    expires_in: Option<String>,
    scope: Option<String>,
    arcade_id: String,
    game_id: String,
) -> TokenResponse {
    TokenResponse {
        access_token,
        token_type,
        expires_in,
        scope,
        arcade_id,
        game_id,
    }
}
