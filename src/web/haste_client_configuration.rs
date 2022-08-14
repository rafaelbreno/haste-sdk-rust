pub struct HasteClientConfiguration {
    pub sign_in_url: String,
}

pub fn new_haste_client_configuration() -> HasteClientConfiguration {
    use crate::utils;
    HasteClientConfiguration {
        sign_in_url: utils::url_build(
            "https".to_string(), 
            "authclient.hastearcade.com".to_string(), 
            0,
        )
    }
}
