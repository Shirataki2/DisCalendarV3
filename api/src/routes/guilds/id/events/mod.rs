pub mod create;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/events").service(create::create));
}
