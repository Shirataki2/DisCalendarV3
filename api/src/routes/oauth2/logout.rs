use crate::prelude::*;

#[get("/logout")]
pub async fn logout(sess: Session) -> Result<HttpResponse, Error> {
    sess.clear();
    let front_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, front_url))
        .finish())
}
