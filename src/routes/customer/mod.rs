mod create_customer;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    create_customer::config(cfg);
}
