use actix_web::{web, App, HttpServer};
use rust_testy::actix::*;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/ei", web::get().to(manual_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    // Ok(())
}
