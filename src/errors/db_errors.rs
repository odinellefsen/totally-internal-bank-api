use crate::http::response::ApiErrorBody;
use actix_web::HttpResponse;
use sqlx::Error;

const CONSTRAINT_CUSTOMER_ID_9_DIGITS: &str = "customer_id_must_be_9_digits_chk";
const CONSTRAINT_CUSTOMER_FIRST_NAME_LEN: &str = "customer_first_name_len_chk";
const CONSTRAINT_CUSTOMER_MIDDLE_NAME_LEN: &str = "customer_middle_name_len_chk";
const CONSTRAINT_CUSTOMER_LAST_NAME_LEN: &str = "customer_last_name_len_chk";
const CONSTRAINT_CUSTOMER_PKEY: &str = "customer_pkey";

pub fn map_db_error(err: &Error) -> HttpResponse {
    if let Error::Database(db_err) = err {
        let sql_state = db_err
            .code()
            .map(|code| code.to_string())
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let constraint = db_err.constraint();

        // 23505: duplicate key violation
        // 23514: check violation
        // 23502: not null violation
        // 22P02: invalid text representation
        // 22007: invalid date format
        // 22008: date exceeded the valid range
        // 22009: date out of range
        // 22015: date is not a valid date

        return match (sql_state.as_str(), constraint) {
            ("23505", Some(CONSTRAINT_CUSTOMER_PKEY)) => conflict("Customer already exists."),
            ("23514", Some(CONSTRAINT_CUSTOMER_ID_9_DIGITS)) => {
                bad_request("Customer ID must be a 9-digit number.")
            }
            ("23514", Some(CONSTRAINT_CUSTOMER_FIRST_NAME_LEN)) => {
                bad_request("First name must be shorter than 150 characters.")
            }
            ("23514", Some(CONSTRAINT_CUSTOMER_MIDDLE_NAME_LEN)) => {
                bad_request("Middle name must be shorter than 250 characters.")
            }
            ("23514", Some(CONSTRAINT_CUSTOMER_LAST_NAME_LEN)) => {
                bad_request("Last name must be shorter than 150 characters.")
            }
            ("23502", _) => bad_request("Missing required field."),
            ("22008", _) => bad_request("Invalid date format. Date exceeded the valid range"),
            ("22007", _) => bad_request("Invalid date format."),
            ("22009", _) => bad_request("Invalid date format. Date out of range"),
            ("22015", _) => bad_request("Invalid date format. Date is not a valid date"),
            _ => internal_error(),
        };
    }

    internal_error()
}

fn bad_request(message: &'static str) -> HttpResponse {
    HttpResponse::BadRequest().json(ApiErrorBody {
        status: 400,
        code: "BAD_REQUEST".to_string(),
        message: message.to_string(),
    })
}

fn conflict(message: &'static str) -> HttpResponse {
    HttpResponse::Conflict().json(ApiErrorBody {
        status: 409,
        code: "CONFLICT".to_string(),
        message: message.to_string(),
    })
}

fn internal_error() -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiErrorBody {
        status: 500,
        code: "INTERNAL_ERROR".to_string(),
        message: "Unexpected error.".to_string(),
    })
}
