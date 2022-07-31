use oauth2::{reqwest::async_http_client, TokenResponse};

use crate::{auth_client::OAuth2Client, prelude::*, routes::get_data};

#[get("/refresh")]
pub async fn refresh(req: HttpRequest, sess: Session) -> Result<HttpResponse, Error> {
    let auth = get_data::<Box<OAuth2Client>>(&req)?;
    let token: AuthToken = match sess.get("google_token")? {
        Some(token) => token,
        None => Err(Error::Unauthorized("Token Not Found".to_string()))?,
    };
    let refresh_token = match token.refresh_token() {
        Some(refresh_token) => refresh_token,
        None => Err(Error::Unauthorized("Refresh Token Not Found".to_string()))?,
    };
    let auth_response = auth
        .get_client()
        .exchange_refresh_token(refresh_token)
        .request_async(async_http_client)
        .await;

    let mut token_body = match auth_response {
        Ok(token_body) => token_body,
        Err(err) => {
            error!("OAuth2 Error: {}", err);
            return Err(Error::Unauthorized(err.to_string()));
        }
    };

    token_body.set_refresh_token(Some(refresh_token.clone()));

    info!("OAuth2 Refresh Success: {:?}", &token_body);
    sess.insert("google_token", &token_body)?;

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish())
}
