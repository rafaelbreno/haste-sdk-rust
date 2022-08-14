pub fn url_build(protocol: String, host: String, port: u16) -> String {
    format!("{}://{}:{}", protocol, host, port).to_string()
}
