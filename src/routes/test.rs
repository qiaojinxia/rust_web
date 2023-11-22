// test.rs
use actix_web::{web, HttpResponse, Responder};

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/hello", web::get().to(hello))
            .route("/are_your_ok", web::post().to(are_your_ok)),
    );
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn are_your_ok() -> impl Responder {
    HttpResponse::Ok().body("are you ok")
}
