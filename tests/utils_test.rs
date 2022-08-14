#[cfg(test)]
mod utils_test {
    use haste::utils::*;

    #[test]
    fn test_url_build() {
        let actual = url_build(
            "https".to_string(), 
            "localhost".to_string(), 
            8080,
        );
        let expected = "https://localhost:8080".to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_url_build_zero_port() {
        let actual = url_build(
            "https".to_string(), 
            "localhost".to_string(), 
            0,
        );
        let expected = "https://localhost".to_string();
        assert_eq!(actual, expected);
    }
}
