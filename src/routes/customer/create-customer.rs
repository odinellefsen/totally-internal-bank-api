use actix_web::{HttpResponse, Responder, web};
use serde::Dezerialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct CreateCustomerRequest {}
