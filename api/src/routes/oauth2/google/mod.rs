pub mod auth;
pub mod callback;
pub mod refresh;
pub mod revoke;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/google")
            .service(callback::process_callback)
            .service(auth::auth)
            .service(revoke::revoke)
            .service(refresh::refresh),
    );
}
