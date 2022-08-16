pub mod create;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let json_cfg = web::JsonConfig::default().limit(12 * 1024 * 1024);
    cfg.service(
        web::scope("/events")
            .app_data(json_cfg)
            .service(create::create),
    );
}
