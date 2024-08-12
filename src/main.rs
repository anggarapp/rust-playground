fn main() {
    // Nothing
}

#[test]
fn test_using_env() {
    use dotenv;
    assert_eq!(
        dotenv::var("TEST_ENV").unwrap(),
        "maidenlesstarnished".to_string()
    );
}

#[tokio::test]
async fn test_connect_db_using_sqlx() -> Result<(), sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    let _pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
        .await?;
    Ok(())
}
