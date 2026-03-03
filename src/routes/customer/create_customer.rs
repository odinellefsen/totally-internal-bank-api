use crate::errors::db_errors::map_db_error;
use crate::http::response::ApiSuccessBody;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
struct CreateCustomerRequest {
    customer_id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct Customer {
    customer_id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: String,
}

async fn create_customer(
    db: web::Data<PgPool>,
    payload: web::Json<CreateCustomerRequest>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Customer>(
        r#"
        INSERT INTO customer (customer_id, first_name, middle_name, last_name, date_of_birth)
        VALUES ($1, $2, NULLIF(BTRIM($3), ''), $4, $5::date)
        RETURNING customer_id, first_name, middle_name, last_name, date_of_birth::text AS date_of_birth
        "#,
    )
    .bind(payload.customer_id)
    .bind(&payload.first_name)
    .bind(&payload.middle_name)
    .bind(&payload.last_name)
    .bind(&payload.date_of_birth)
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(customer) => HttpResponse::Created().json(ApiSuccessBody {
            status: 201,
            code: "SUCCESS".to_string(),
            message: "Customer created".to_string(),
            data: customer,
        }),
        Err(err) => map_db_error(&err),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/customers", web::post().to(create_customer));
}
