use actix_web::web;

use crate::temperatures;

pub fn config(cfg: &mut web::ServiceConfig) {
    temperatures::routes::init_routes(cfg);
}
