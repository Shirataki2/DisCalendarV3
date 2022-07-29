pub mod callback;
pub mod login;
pub mod logout;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/oauth2")
            //.route("/authorize", web::get().to(oauth2::authorize))
            .service(callback::process_callback)
            .service(login::login)
            .service(logout::logout),
    );
}
