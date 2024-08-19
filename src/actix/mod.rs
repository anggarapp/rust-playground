use actix_web::{get, post, HttpResponse, Responder};

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
