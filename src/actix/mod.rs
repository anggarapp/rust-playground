mod model;
use actix_web::{get, post, web, HttpResponse, Responder};
use model::{TestCreate, TestModel, TestModelResponse};
use sqlx::postgres::PgPool;

pub struct AppState {
    pub db: PgPool,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Kiddos")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_handler() -> impl Responder {
    HttpResponse::Ok().body("Manual Handler")
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
pub async fn get_test_row(data: web::Data<AppState>) -> impl Responder {
    let test_vec: Vec<TestModel> = sqlx::query_as("select * from test")
        .fetch_all(&data.db)
        .await
        .unwrap();

    let test_response = test_vec
        .into_iter()
        .map(|test| filter_db_record(&test))
        .collect::<Vec<TestModelResponse>>();

    let test_json = serde_json::json!({
        "status": "success",
        "results": test_response.len(),
        "test": test_response
    });

    HttpResponse::Ok().json(test_json)
}

#[post("/")]
pub async fn create_test_row(
    data: web::Data<AppState>,
    body: web::Json<TestCreate>,
) -> impl Responder {
    let query = sqlx::query("INSERT INTO test (place) VALUES ($1)")
        .bind(&body.place)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());
    match query {
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
        }
        Ok(_) => {
            return HttpResponse::Ok().into();
        }
    }
}

#[get("/{id}")]
pub async fn get_test_row_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let test_id = path.into_inner();
    let test: TestModel = sqlx::query_as("select * from test where id = $1")
        .bind(test_id)
        .fetch_one(&data.db)
        .await
        .unwrap();

    let test_response = filter_db_record(&test);

    let test_json = serde_json::json!({
        "status": "success",
        "test": test_response
    });

    HttpResponse::Ok().json(test_json)
}

fn filter_db_record(test: &TestModel) -> TestModelResponse {
    TestModelResponse {
        id: test.id.to_owned(),
        place: test.place.to_owned(),
        createdAt: test.created_at.unwrap(),
        updatedAt: test.updated_at.unwrap(),
    }
}
