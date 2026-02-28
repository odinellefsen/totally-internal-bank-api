use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

async fn health_check(db: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query("SELECT 1").execute(db.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("ok"),
        Err(_) => HttpResponse::InternalServerError().json("db not ready"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}
