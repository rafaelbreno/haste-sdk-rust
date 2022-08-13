#[cfg(test)]
mod models_test {
    #[test]
    fn test_new_arcade() {
        let result = haste::models::arcade::new_arcade("some_id".to_string());
        let expected = haste::models::arcade::Arcade{id:"some_id".to_string(), name: "".to_string(), description: "".to_string()};
        assert_eq!(result, expected);
    }
}
