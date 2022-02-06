use actix_web::web;

use crate::measurements;
use crate::nodes;

pub fn config(cfg: &mut web::ServiceConfig) {
    nodes::routes::init_routes(cfg);
    measurements::routes::init_routes(cfg);
}
