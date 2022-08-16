use actix_web::web;
use actix_web::HttpRequest;

use crate::error::create_error;

pub mod guilds;
pub mod oauth2;
pub mod payment;
pub mod users;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(oauth2::init_routes);
    cfg.configure(users::init_routes);
    cfg.configure(guilds::init_routes);
    cfg.configure(payment::init_routes);
}

pub fn get_data<T>(req: &HttpRequest) -> Result<&T, crate::error::Error>
where
    T: 'static,
{
    match req.app_data::<T>() {
        Some(data) => Ok(data),
        None => {
            error!("Failed to get application data: {}", stringify!(T));
            Err(create_error("Internal Server Setup Failed"))
        }
    }
}
