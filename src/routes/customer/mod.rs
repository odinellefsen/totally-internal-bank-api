mod create_customer;
mod delete_customer;
mod update_customer;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    create_customer::config(cfg);
    update_customer::config(cfg);
    delete_customer::config(cfg);
}
