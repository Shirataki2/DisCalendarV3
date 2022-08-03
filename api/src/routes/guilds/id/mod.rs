pub mod channels;
pub mod events;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/{id}")
            .service(channels::channels)
            .configure(events::init_routes),
    );
}
