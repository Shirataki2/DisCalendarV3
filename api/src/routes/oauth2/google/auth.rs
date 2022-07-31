use crate::{auth_client::OAuth2Client, prelude::*, routes::get_data};

#[get("/auth")]
pub async fn auth(req: HttpRequest, sess: Session) -> Result<HttpResponse, Error> {
    let auth = get_data::<Box<OAuth2Client>>(&req)?;

    let pkce = auth.create_pkce_auth_url(&[("access_type", "offline")]);
    sess.insert("google_verifier", pkce.verifier)?;
    sess.insert("google_csrf_token", pkce.csrf_token)?;

    let url = pkce.auth_url.to_string();
    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, url))
        .finish())
}
