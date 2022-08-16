pub mod checkout;
pub mod list_donate;
pub mod register;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payment")
            .service(list_donate::list_donate)
            .service(checkout::checkout)
            .service(register::register),
    );
}
