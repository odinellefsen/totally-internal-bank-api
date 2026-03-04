use crate::errors::db_errors::map_db_error;
use crate::http::response::{ApiErrorBody, ApiSuccessBody};
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
struct DeleteCustomerPath {
    customer_id: i32,
}

#[derive(Serialize, sqlx::FromRow)]
struct Customer {
    customer_id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: String,
}

async fn delete_customer(
    db: web::Data<PgPool>,
    path: web::Path<DeleteCustomerPath>,
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
        FROM delete_customer($1::integer)
        "#,
        path.customer_id,
    )
    .fetch_optional(db.get_ref())
    .await;

    match result {
        Ok(Some(customer)) => HttpResponse::Ok().json(ApiSuccessBody {
            status: 200,
            code: "SUCCESS".to_string(),
            message: "Customer deleted.".to_string(),
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
    cfg.route(
        "/customers/{customer_id}",
        web::delete().to(delete_customer),
    );
}
