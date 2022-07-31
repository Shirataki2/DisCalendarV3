use crate::{
    auth_client::OAuth2Client,
    discord::Client,
    error::Error,
    routes::{
        get_data,
        oauth2::{get_auth_code, OAuth2Query},
    },
};
use actix_session::Session;
use actix_web::{http::header, web, HttpRequest, HttpResponse};
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, PkceCodeVerifier};

#[get("/callback")]
async fn process_callback(
    req: HttpRequest,
    sess: Session,
    query: web::Query<OAuth2Query>,
) -> Result<HttpResponse, Error> {
    let (code, state) = match get_auth_code(&query) {
        Some((code, state)) => (code, state),
        None => {
            return Ok(HttpResponse::Found()
                .append_header((header::LOCATION, "/canceled"))
                .finish())
        }
    };
    info!("OAuth2 Success: code={} state={}", &code, &state);

    // Check CSRF Token to prevent CSRF attacks

    let code = AuthorizationCode::new(code);
    let state = CsrfToken::new(state);
    let csrf_token = match sess.get::<CsrfToken>("csrf_token")? {
        Some(token) => token,
        None => return Err(Error::Unauthorized("CSRF Token Not Found".to_string())),
    };
    if state.secret() != csrf_token.secret() {
        return Err(Error::Unauthorized("CSRF Token Mismatch".to_string()));
    }
    let pkce_verifier = match sess.get::<PkceCodeVerifier>("verifier")? {
        Some(verifier) => verifier,
        None => {
            return Err(Error::Unauthorized(
                "PKCE Code Verifier Not Found".to_string(),
            ))
        }
    };

    // Fetch the access token from the authorization code.

    let auth = get_data::<OAuth2Client>(&req)?;
    let auth_response = auth
        .get_client()
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    let token_body = match auth_response {
        Ok(token_body) => token_body,
        Err(err) => {
            error!("OAuth2 Error: {}", err);
            return Err(Error::Unauthorized(err.to_string()));
        }
    };

    info!("OAuth2 Success: {:?}", &token_body);
    sess.insert("token", &token_body)?;

    let client = Client::from_token(&token_body)?;
    let user = client.fetch_current_user().await?;

    sess.insert("user", &user)?;

    // Redirect to frontend with user info
    let front_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, front_url))
        .finish())
}
