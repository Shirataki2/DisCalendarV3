use crate::prelude::*;

#[post("/")]
async fn create() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(""))
}
