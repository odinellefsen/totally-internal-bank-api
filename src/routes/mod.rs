pub mod health;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    health::config(cfg)
}
