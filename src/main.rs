use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TestModel {
    pub id: i32,
    pub place: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
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

#[tokio::test]
async fn test_select_one_using_sqlx() -> Result<(), sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    let _pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_select_all_using_sqlx() -> Result<(), sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
        .await?;
    let row: TestModel = sqlx::query_as("select * from test where id = $1")
        .bind(1)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.id, 1);
    assert_eq!(row.place, "test place");
    println!("id is {} and place is {}", &row.id, &row.place);
    Ok(())
}

#[tokio::test]
async fn test_insert_one_using_sqlx() -> Result<(), sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    let _pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
        .await?;
    sqlx::query("INSERT INTO test (place) VALUES ($1)")
        .bind("Sotgad".to_string())
        .execute(&_pool)
        .await?;
    let row: TestModel = sqlx::query_as("select * from test where place = $1")
        .bind("Sotgad".to_string())
        .fetch_one(&_pool)
        .await?;
    assert_eq!(row.place, "Sotgad");
    Ok(())
}

#[tokio::test]
async fn test_update_row_using_sqlx() -> Result<(), sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    let _pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
        .await?;
    let the_id: (i32,) = sqlx::query_as("INSERT INTO test (place) VALUES ($1) RETURNING id")
        .bind("Notgauard".to_string())
        .fetch_one(&_pool)
        .await?;
    let updated_row: TestModel =
        sqlx::query_as("UPDATE test set place = $1 where id = $2 RETURNING *")
            .bind("Notgat".to_string())
            .bind(the_id.0)
            .fetch_one(&_pool)
            .await?;
    assert_eq!(updated_row.id, the_id.0);
    assert_eq!(updated_row.place, "Notgat");
    Ok(())
}

#[tokio::test]
async fn test_delete_row_using_sqlx() -> Result<(), sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    let _pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
        .await?;
    let the_delete: (i32,) = sqlx::query_as("INSERT INTO test (place) VALUES ($1) RETURNING id")
        .bind("Notgauard".to_string())
        .fetch_one(&_pool)
        .await?;
    let row: Vec<TestModel> = sqlx::query_as("select * from test where id = $1")
        .bind(&the_delete.0)
        .fetch_all(&_pool)
        .await?;
    assert!(row.len() > 0 as usize);
    sqlx::query("delete from test where id = $1")
        .bind(&the_delete.0)
        .execute(&_pool)
        .await?;
    let row_del: Vec<TestModel> = sqlx::query_as("select * from test where id = $1")
        .bind(&the_delete.0)
        .fetch_all(&_pool)
        .await?;
    assert!(row_del.len() <= 0 as usize);
    Ok(())
}

#[tokio::test]
async fn test_transaction_success_using_sqlx() -> Result<(), sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
        .await?;
    let row_first: Vec<TestModel> = sqlx::query_as("select * from test")
        .fetch_all(&pool)
        .await?;

    {
        let mut tx = pool.begin().await?;
        let the_id: (i32,) = sqlx::query_as("INSERT INTO test (place) VALUES ($1) RETURNING id")
            .bind("Notgauard".to_string())
            .fetch_one(&mut *tx)
            .await?;

        let _updated_row: TestModel =
            sqlx::query_as("UPDATE test set place = $1 where id = $2 RETURNING *")
                .bind("Notgat".to_string())
                .bind(&the_id.0)
                .fetch_one(&mut *tx)
                .await?;
        if 0 < 0 {
            tx.rollback().await?;
        } else {
            tx.commit().await?
        }
    }
    let row_latest: Vec<TestModel> = sqlx::query_as("select * from test")
        .fetch_all(&pool)
        .await?;
    assert_ne!(row_first.len(), row_latest.len());
    Ok(())
}
#[tokio::test]
async fn test_transaction_failed_using_sqlx() -> Result<(), sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
        .await?;
    let row_first: Vec<TestModel> = sqlx::query_as("select * from test")
        .fetch_all(&pool)
        .await?;

    {
        let mut tx = pool.begin().await?;
        let the_id: (i32,) = sqlx::query_as("INSERT INTO test (place) VALUES ($1) RETURNING id")
            .bind("Notgauard".to_string())
            .fetch_one(&mut *tx)
            .await?;

        let _updated_row: TestModel =
            sqlx::query_as("UPDATE test set place = $1 where id = $2 RETURNING *")
                .bind("Notgat".to_string())
                .bind(&the_id.0)
                .fetch_one(&mut *tx)
                .await?;
        if 0 < 1 {
            tx.rollback().await?;
        } else {
            tx.commit().await?
        }
    }
    let row_latest: Vec<TestModel> = sqlx::query_as("select * from test")
        .fetch_all(&pool)
        .await?;
    assert_eq!(row_first.len(), row_latest.len());
    Ok(())
}
