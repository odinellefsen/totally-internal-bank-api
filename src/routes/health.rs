use actix_web::{HttpResponse, Responder, web};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check))
}
