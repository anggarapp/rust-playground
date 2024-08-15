#[test]
fn test_using_env() {
    use dotenv;
    assert_eq!(
        dotenv::var("TEST_ENV").unwrap(),
        "maidenlesstarnished".to_string()
    );
}
