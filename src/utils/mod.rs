pub fn url_build(protocol: String, host: String, port: u16) -> String {
    match port {
        0 => format!("{}://{}", protocol, host).to_string(),
        _ => format!("{}://{}:{}", protocol, host, port).to_string()
    }
}
