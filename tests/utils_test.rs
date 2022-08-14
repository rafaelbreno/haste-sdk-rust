#[cfg(test)]
mod utils_test {
    #[test]
    fn test_url_build() {
        use haste::utils::url_build;
        let actual = url_build(
            "https".to_string(), 
            "localhost".to_string(), 
            8080,
        );
        let expected = "https://localhost:8080".to_string();
        assert_eq!(actual, expected);
    }
}
