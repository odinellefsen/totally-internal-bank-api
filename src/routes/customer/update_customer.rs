use crate::errors::db_errors::map_db_error;
use crate::http::response::ApiSuccessBody;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
struct UpdateCustomerRequest {
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
    let result = sqlx::query_as!(
        Customer,
        r#"
        SELECT
            customer_id as "customer_id!",
            first_name as "first_name!",
            middle_name,
            last_name as "last_name!",
            date_of_birth::text as "date_of_birth!"
        FROM create_customer($1, $2, $3, $4, to_date($5, 'YYYY-MM-DD'))
        "#,
        payload.customer_id,
        payload.first_name.as_str(),
        payload.middle_name.as_deref(),
        payload.last_name.as_str(),
        payload.date_of_birth.as_str(),
    )
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(customer) => HttpResponse::Created().json(ApiSuccessBody {
            status: 201,
            code: "SUCCESS".to_string(),
            message: "Customer created.".to_string(),
            data: customer,
        }),
        Err(err) => map_db_error(&err),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/customers", web::post().to(create_customer));
}
