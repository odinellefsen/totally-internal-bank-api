use crate::http::response::ApiErrorBody;
use actix_web::HttpResponse;
use sqlx::Error;
use sqlx::postgres::PgDatabaseError;

// 23505: duplicate key violation
// 23514: check violation
// 23502: not null violation
// 23503: foreign key violation
// 22P02: invalid text representation
// 22007: invalid date format
// 22008: date exceeded the valid range
// 22009: date out of range
// 22015: date is not a valid date

pub fn map_db_error(err: &Error) -> HttpResponse {
    if let Error::Database(db_err) = err {
        let sql_state_code = db_err.code();
        let sql_state = sql_state_code.as_deref().unwrap_or("UNKNOWN");
        let constraint = db_err.constraint();
        let table = db_err.table();
        let column = db_err
            .try_downcast_ref::<PgDatabaseError>()
            .and_then(|pg_err| pg_err.column()); // postgres-only

        if let Some(response) = map_customer_constraint_error(sql_state, constraint, table, column)
        {
            return response;
        }

        if let Some(response) = map_generic_database_error(sql_state) {
            return response;
        }
    }

    internal_error()
}

fn map_customer_constraint_error(
    sql_state: &str,
    constraint: Option<&str>,
    table: Option<&str>,
    column: Option<&str>,
) -> Option<HttpResponse> {
    const CONSTRAINT_CUSTOMER_ID_9_DIGITS: &str = "customer_id_must_be_9_digits_chk";
    const CONSTRAINT_CUSTOMER_FIRST_NAME_LEN: &str = "customer_first_name_len_chk";
    const CONSTRAINT_CUSTOMER_MIDDLE_NAME_LEN: &str = "customer_middle_name_len_chk";
    const CONSTRAINT_CUSTOMER_LAST_NAME_LEN: &str = "customer_last_name_len_chk";
    const CONSTRAINT_CUSTOMER_PKEY: &str = "customer_pkey";

    match (sql_state, constraint, table, column) {
        ("23502", _, Some("customer"), Some("first_name")) => {
            // first_name should be normalized with NULLIF(BTRIM(...), '') before insert.
            // So for customer.first_name, 23502 effectively means the input was empty/whitespace.
            // Because API endpoint should be guarded with String and not Option<String>
            // which prevents null being passed in directly.
            Some(bad_request("First name cannot be empty nor only spaces."))
        }
        ("23502", _, Some("customer"), Some("last_name")) => {
            // last_name should be normalized with NULLIF(BTRIM(...), '') before insert.
            // So for customer.last_name, 23502 effectively means the input was empty/whitespace.
            // Because API endpoint should be guarded with String and not Option<String>
            // which prevents null being passed in directly.
            Some(bad_request("Last name cannot be empty nor only spaces."))
        }
        ("23505", Some(CONSTRAINT_CUSTOMER_PKEY), _, _) => {
            Some(conflict("Customer with that SSN already exists."))
        }
        ("23514", Some(CONSTRAINT_CUSTOMER_ID_9_DIGITS), _, _) => Some(bad_request(
            "Customer ID must be a 9-digit number. It represents the SSN of the customer.",
        )),
        ("23514", Some(CONSTRAINT_CUSTOMER_FIRST_NAME_LEN), _, _) => Some(bad_request(
            "First name must be shorter than 150 characters.",
        )),
        ("23514", Some(CONSTRAINT_CUSTOMER_MIDDLE_NAME_LEN), _, _) => Some(bad_request(
            "Middle name must be shorter than 250 characters.",
        )),
        ("23514", Some(CONSTRAINT_CUSTOMER_LAST_NAME_LEN), _, _) => Some(bad_request(
            "Last name must be shorter than 150 characters.",
        )),
        _ => None,
    }
}

fn map_generic_database_error(sql_state: &str) -> Option<HttpResponse> {
    match sql_state {
        "23505" => Some(conflict(
            "A record with one of these values already exists.",
        )),
        "23514" => Some(bad_request("Input failed a database validation check.")),
        "23502" => Some(bad_request("Missing required field.")),
        "23503" => Some(bad_request("Referenced record does not exist.")),
        "22P02" => Some(bad_request("Invalid value format for one of the fields.")),
        "22008" => Some(bad_request(
            "Invalid date format. The format is YYYY-MM-DD.",
        )),
        "22007" => Some(bad_request(
            "Invalid date format. The format is YYYY-MM-DD.",
        )),
        "22009" => Some(bad_request(
            "Invalid date format. The format is YYYY-MM-DD.",
        )),
        "22015" => Some(bad_request(
            "Invalid date format. The format is YYYY-MM-DD.",
        )),
        _ => None,
    }
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
