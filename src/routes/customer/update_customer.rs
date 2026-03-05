use crate::errors::db_errors::map_db_error;
use crate::http::response::{ApiErrorBody, ApiSuccessBody};
use actix_web::{HttpResponse, Responder, web};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
struct UpdateCustomerRequest {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    date_of_birth: Option<String>,
}

#[derive(Deserialize)]
struct UpdateCustomerPath {
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

#[derive(sqlx::FromRow)]
struct UpdateCustomerRow {
    old_customer_id: i32,
    old_first_name: String,
    old_middle_name: Option<String>,
    old_last_name: String,
    old_date_of_birth: String,
    new_customer_id: i32,
    new_first_name: String,
    new_middle_name: Option<String>,
    new_last_name: String,
    new_date_of_birth: String,
}

#[derive(Serialize)]
struct UpdateCustomerResponse {
    before: Customer,
    after: Customer,
}

async fn update_customer(
    db: web::Data<PgPool>,
    path: web::Path<UpdateCustomerPath>,
    payload: web::Json<UpdateCustomerRequest>,
) -> impl Responder {
    if payload.first_name.is_none()
        && payload.middle_name.is_none()
        && payload.last_name.is_none()
        && payload.date_of_birth.is_none()
    {
        return HttpResponse::BadRequest().json(ApiErrorBody {
            status: 400,
            code: "BAD_REQUEST".to_string(),
            message: "At least one field must be provided to update.".to_string(),
        });
    }

    if let Some(date_of_birth) = payload.date_of_birth.as_deref() {
        if NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").is_err() {
            return HttpResponse::BadRequest().json(ApiErrorBody {
                status: 400,
                code: "BAD_REQUEST".to_string(),
                message: "Invalid date format. The format is YYYY-MM-DD.".to_string(),
            });
        }
    }

    let result = sqlx::query_as!(
        UpdateCustomerRow,
        r#"
        SELECT
            old_customer_id as "old_customer_id!",
            old_first_name as "old_first_name!",
            old_middle_name,
            old_last_name as "old_last_name!",
            old_date_of_birth as "old_date_of_birth!",
            new_customer_id as "new_customer_id!",
            new_first_name as "new_first_name!",
            new_middle_name,
            new_last_name as "new_last_name!",
            new_date_of_birth as "new_date_of_birth!"
        FROM update_customer(
            $1::integer,
            $2::varchar(150),
            $3::varchar(250),
            $4::varchar(150),
            CASE
                WHEN $5::text IS NULL THEN NULL
                ELSE to_date($5::text, 'YYYY-MM-DD')
            END
        )
        "#,
        path.customer_id,
        payload.first_name.as_deref(),
        payload.middle_name.as_deref(),
        payload.last_name.as_deref(),
        payload.date_of_birth.as_deref(),
    )
    .fetch_optional(db.get_ref())
    .await;

    match result {
        Ok(Some(row)) => HttpResponse::Ok().json(ApiSuccessBody {
            status: 200,
            code: "SUCCESS".to_string(),
            message: "Customer updated.".to_string(),
            data: UpdateCustomerResponse {
                before: Customer {
                    customer_id: row.old_customer_id,
                    first_name: row.old_first_name,
                    middle_name: row.old_middle_name,
                    last_name: row.old_last_name,
                    date_of_birth: row.old_date_of_birth,
                },
                after: Customer {
                    customer_id: row.new_customer_id,
                    first_name: row.new_first_name,
                    middle_name: row.new_middle_name,
                    last_name: row.new_last_name,
                    date_of_birth: row.new_date_of_birth,
                },
            },
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
    cfg.route("/customers/{customer_id}", web::patch().to(update_customer));
}
