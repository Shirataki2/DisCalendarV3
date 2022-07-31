use crate::{auth_client::OAuth2Client, prelude::*, routes::get_data};

#[get("/revoke")]
pub async fn revoke(req: HttpRequest, sess: Session) -> Result<HttpResponse, Error> {
    let auth = get_data::<Box<OAuth2Client>>(&req)?;
    let token = sess.get("google_token")?;
    if let Some(token) = token {
        auth.revoke(token).await?;
    }
    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish())
}
