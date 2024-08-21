use actix_web::{web, App, HttpServer};
use rust_testy::actix::*;
use sqlx::postgres::PgPoolOptions;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = match PgPoolOptions::new()
        .max_connections(3)
        .connect(dotenv::var("TEST_DB_STRING").unwrap().as_str())
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        let init_scope = web::scope("/init")
            .service(hello)
            .service(echo)
            .route("/ei", web::get().to(manual_handler));
        let test_scope = web::scope("/test")
            .service(health_check)
            .service(get_test_row)
            .service(get_test_row_by_id)
            .service(create_test_row);

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(init_scope)
            .service(test_scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    // Ok(())
}
