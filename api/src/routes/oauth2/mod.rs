pub mod callback;
pub mod google;
pub mod login;
pub mod logout;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/oauth2")
            //.route("/authorize", web::get().to(oauth2::authorize))
            .service(callback::process_callback)
            .service(login::login)
            .service(logout::logout)
            .configure(google::init_routes),
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuth2Query {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub fn get_auth_code(query: &web::Query<OAuth2Query>) -> Option<(String, String)> {
    match (&query.code, &query.state) {
        (Some(code), Some(state)) => Some((code.clone(), state.clone())),
        _ => {
            info!(
                "OAuth2 Error: {:?} {:?}",
                query.error, query.error_description
            );
            None
        }
    }
}
