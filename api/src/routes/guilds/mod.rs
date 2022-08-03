pub mod id;
pub mod list;
pub mod tmp;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/guilds")
            .service(list::guild_list)
            .service(tmp::tmp)
            .configure(id::init_routes),
    );
}
