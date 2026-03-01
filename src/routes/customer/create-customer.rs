use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct CreateCustomerRequest {
    customer_id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: Option<String>,
}

async fn create_customer(
    db: web::Data<PgPool>,
    payload: web::Json<CreateCustomerRequest>,
) -> impl Responder {
    let result = sqlx::query(
        r#"
        INSERT INTO customer (customer_id, first_name, middle_name, last_name, date_of_birth)
        VALUES ($1, $2, $3, $4, $5::date)
        "#,
    )
    .bind(payload.customer_id)
    .bind(&payload.first_name)
    .bind(&payload.middle_name)
    .bind(&payload.last_name)
    .bind(&payload.date_of_birth)
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json("customer created"),
        Err(err) => HttpResponse::BadRequest().json(format!("insert failed: {err}")),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/customers", web::post().to(create_customer));
}
