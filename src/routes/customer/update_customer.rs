use crate::errors::db_errors::map_db_error;
use crate::http::response::{ApiErrorBody, ApiSuccessBody};
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

async fn update_customer(
    db: web::Data<PgPool>,
    payload: web::Json<UpdateCustomerRequest>,
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
        FROM update_customer(
            $1::integer,
            $2::varchar(150),
            $3::varchar(250),
            $4::varchar(150),
            to_date($5, 'YYYY-MM-DD')
        )
        "#,
        payload.customer_id,
        payload.first_name.as_str(),
        payload.middle_name.as_deref(),
        payload.last_name.as_str(),
        payload.date_of_birth.as_str(),
    )
    .fetch_optional(db.get_ref())
    .await;

    match result {
        Ok(Some(customer)) => HttpResponse::Ok().json(ApiSuccessBody {
            status: 200,
            code: "SUCCESS".to_string(),
            message: "Customer updated.".to_string(),
            data: customer,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiErrorBody {
            status: 404,
            code: "NOT_FOUND".to_string(),
            message: "Customer not found.".to_string(),
        }),
        Err(err) => map_db_error(&err),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/customers", web::put().to(update_customer));
}
