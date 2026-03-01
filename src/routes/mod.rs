pub mod health;
pub mod customer;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    health::config(cfg);
    customer::config(cfg);
}
