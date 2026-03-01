use actix_web::{HttpResponse, Responder, web};
use serde::Dezerialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct CreateCustomerRequest {
    customer_id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: Option<String>,
}
