use actix_web::web;

use crate::measurements;

pub fn config(cfg: &mut web::ServiceConfig) {
    measurements::routes::init_routes(cfg);
}
