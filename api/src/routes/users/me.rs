use actix_session::Session;
use actix_web::HttpResponse;

use crate::{discord::Client, error::Error, serenity_models};

#[get("/me")]
pub async fn me(sess: Session) -> Result<HttpResponse, Error> {
    let cached_user = sess.get::<serenity_models::CurrentUser>("user")?;
    if cached_user.is_some() {
        return Ok(HttpResponse::Ok().json(cached_user.unwrap()));
    }
    let auth_token = sess.get::<crate::AuthToken>("token")?;
    if auth_token.is_none() {
        return Err(Error::Unauthorized("Token Not Found".to_string()));
    }
    let client = Client::from_token(&auth_token.unwrap())?;
    let user = client.fetch_current_user().await?;
    sess.insert("user", &user)?;
    Ok(HttpResponse::Ok().json(user))
}
