use actix_web::HttpResponse;
use serde::Serialize;
use sqlx::Error;

const CONSTRAINT_CUSTOMER_ID_9_DIGITS: &str = "customer_id_must_be_9_digits_chk";
const CONSTRAINT_CUSTOMER_FIRST_NAME_LEN: &str = "customer_first_name_len_chk";
const CONSTRAINT_CUSTOMER_MIDDLE_NAME_LEN: &str = "customer_middle_name_len_chk";
const CONSTRAINT_CUSTOMER_LAST_NAME_LEN: &str = "customer_last_name_len_chk";
const CONSTRAINT_CUSTOMER_PKEY: &str = "customer_pkey";

#[derive(Serialize)]
struct ApiErrorBody<'a> {
    status: u16,
    code: &'a str,
    message: &'a str,
}

pub fn map_db_error(err: &Error) -> HttpResponse {
    if let Error::Database(db_err) = err {
        let sql_state = db_err
            .code()
            .map(|code| code.to_string())
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let constraint = db_err.constraint();

        let sql_state_code = sql_state_code_mapper(sql_state.as_str());

        return match (sql_state_code, constraint) {
            ("CONFLICT", Some(CONSTRAINT_CUSTOMER_PKEY)) => conflict("Customer already exists."),
            ("BAD_REQUEST", Some(CONSTRAINT_CUSTOMER_ID_9_DIGITS)) => {
                bad_request("Customer ID must be a 9-digit number.")
            }
            ("BAD_REQUEST", Some(CONSTRAINT_CUSTOMER_FIRST_NAME_LEN)) => {
                bad_request("First name must be shorter than 150 characters.")
            }
            ("BAD_REQUEST", Some(CONSTRAINT_CUSTOMER_MIDDLE_NAME_LEN)) => {
                bad_request("Middle name must be shorter than 250 characters.")
            }
            ("BAD_REQUEST", Some(CONSTRAINT_CUSTOMER_LAST_NAME_LEN)) => {
                bad_request("Last name must be shorter than 150 characters.")
            }
            ("BAD_REQUEST", _) => bad_request("Missing required field."),
            _ => internal_error(),
        };
    }

    internal_error()
}

fn sql_state_code_mapper(sql_state: &str) -> &str {
    match sql_state {
        "23505" => "CONFLICT",
        "23514" => "BAD_REQUEST",
        "22P02" => "BAD_REQUEST",
        _ => "INTERNAL_ERROR",
    }
}

fn bad_request(message: &'static str) -> HttpResponse {
    HttpResponse::BadRequest().json(ApiErrorBody {
        status: 400,
        code: "BAD_REQUEST",
        message,
    })
}

fn conflict(message: &'static str) -> HttpResponse {
    HttpResponse::Conflict().json(ApiErrorBody {
        status: 409,
        code: "CONFLICT",
        message,
    })
}

fn internal_error() -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiErrorBody {
        status: 500,
        code: "INTERNAL_ERROR",
        message: "Unexpected error.",
    })
}
