#[cfg(test)]
mod test {
    use crate::actix::model::TestResponses;
    use crate::actix::*;
    use actix_web::{
        http::{self},
        test, App,
    };
    use model::TestResponse;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{Pool, Postgres};

    async fn get_default_pool() -> Pool<Postgres> {
        match PgPoolOptions::new()
            .max_connections(3)
            .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
            .await
        {
            Ok(pool) => pool,
            Err(_) => {
                std::process::exit(1);
            }
        }
    }

    #[actix_web::test]
    async fn test_api_get_all() {
        // init database pool
        let pool = get_default_pool().await;
        // querying row count
        let row_count: Result<i64, sqlx::Error> = sqlx::query_scalar("select count(*) from test")
            .fetch_one(&pool)
            .await;
        // querying all records
        let row_result: Vec<TestModel> = sqlx::query_as("select * from test")
            .fetch_all(&pool)
            .await
            .unwrap();
        // convert to model
        let test_results = row_result
            .into_iter()
            .map(|test| filter_db_record(&test))
            .collect::<Vec<TestModelResponse>>();
        // init app
        let app = App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(get_test_row);
        // init test
        let mut app = test::init_service(app).await;
        // create request
        let req = test::TestRequest::get().uri("/").to_request();
        // create response
        let resp = test::call_service(&mut app, req).await;
        // assert
        assert_eq!(resp.status(), http::StatusCode::OK);

        let resp_body = test::read_body(resp).await;
        let json_resp: TestResponses = serde_json::from_slice(&resp_body).unwrap();
        assert_eq!(json_resp.status, "success".to_string());
        assert_eq!(json_resp.results as i64, row_count.unwrap());
        assert_eq!(json_resp.test, test_results);
    }
    #[actix_web::test]
    async fn test_api_get_by_id() {
        // init database pool
        let pool = get_default_pool().await;
        // insert new row and retrieve id
        let row_id: i32 = sqlx::query_scalar("INSERT INTO test (place) VALUES ($1) RETURNING id")
            .bind("Notgauard".to_string())
            .fetch_one(&pool)
            .await
            .unwrap();
        // retrieve row by id
        let row_query: TestModel = sqlx::query_as("select * from test where id = $1")
            .bind(&row_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        let row = filter_db_record(&row_query);
        let app = App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(get_test_row_by_id);
        let mut app = test::init_service(app).await;
        let uri = ["/", &row_id.to_string()].join("");
        let req = test::TestRequest::get().uri(&uri).to_request();
        let res = test::call_service(&mut app, req).await;
        assert_eq!(res.status(), http::StatusCode::OK);
        let res_body = test::read_body(res).await;
        let json_res: TestResponse = serde_json::from_slice(&res_body).unwrap();
        assert_eq!(json_res.status, "success".to_string());
        assert_eq!(json_res.test, row);
    }
    #[actix_web::test]
    async fn test_api_create() {
        let pool = get_default_pool().await;
        let app = App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(create_test_row);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(TestCreate {
                place: "test its".to_string(),
            })
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert_eq!(res.status(), http::StatusCode::OK);
        let res_body = test::read_body(res).await;
        let json_res: TestResponse = serde_json::from_slice(&res_body).unwrap();
        let test_inserted = json_res.test;
        let inserted_id = &test_inserted.id;
        let row_query: TestModelResponse = sqlx::query_as("select * from test where id = $1")
            .bind(&inserted_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(test_inserted, row_query);
    }
    #[actix_web::test]
    async fn test_api_update() {
        let pool = get_default_pool().await;
        let insert_row: TestModel =
            sqlx::query_as("INSERT INTO test (place) VALUES ($1) RETURNING *")
                .bind("Sotgauard".to_string())
                .fetch_one(&pool)
                .await
                .unwrap();
        let uri = ["/", &insert_row.id.to_string()].join("");
        let inserted_row = filter_db_record(&insert_row);
        let app = App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(update_test_row);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::put()
            .uri(&uri)
            .set_json(TestCreate {
                place: "test its update".to_string(),
            })
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert_eq!(res.status(), http::StatusCode::OK);
        let res_body = test::read_body(res).await;
        let json_res: TestResponse = serde_json::from_slice(&res_body).unwrap();
        let test_inserted = json_res.test;
        assert_ne!(inserted_row, test_inserted);
        assert_eq!(test_inserted.place, "test its update".to_string());
    }
    #[actix_web::test]
    async fn test_api_delete() {
        let pool = get_default_pool().await;
        let insert_row: TestModel =
            sqlx::query_as("INSERT INTO test (place) VALUES ($1) RETURNING *")
                .bind("row to delete".to_string())
                .fetch_one(&pool)
                .await
                .unwrap();
        let uri = ["/", &insert_row.id.to_string()].join("");
        let app = App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(delete_test_row);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::delete().uri(&uri).to_request();
        let res = test::call_service(&mut app, req).await;
        assert_eq!(res.status(), http::StatusCode::OK);

        let deleted_query: Result<TestModel, sqlx::Error> =
            sqlx::query_as("select * from test where id")
                .bind("row to delete".to_string())
                .fetch_one(&pool)
                .await;
        match deleted_query {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }
}
