use crate::errors::db_errors::map_db_error;
use crate::http::response::{ApiErrorBody, ApiSuccessBody};
use actix_web::{HttpResponse, Responder, web};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
struct CreateAccountRequest {
    customer_id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct Account {
    customer_id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    date_of_birth: String,
}

async fn create_account(
    db: web::Data<PgPool>,
    payload: web::Json<CreateAccountRequest>,
) -> impl Responder {
    if NaiveDate::parse_from_str(payload.date_of_birth.as_str(), "%Y-%m-%d").is_err() {
        return HttpResponse::BadRequest().json(ApiErrorBody {
            status: 400,
            code: "BAD_REQUEST".to_string(),
            message: "Invalid date format. The format is YYYY-MM-DD.".to_string(),
        });
    }

    let result = sqlx::query_as!(
        Account,
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
    cfg.route("/customers", web::post().to(create_account));
}
