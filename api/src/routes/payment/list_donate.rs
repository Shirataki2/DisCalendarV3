use anyhow::Context;

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StripeProducts {
    pub products: stripe::List<stripe::Product>,
    pub prices: stripe::List<stripe::Price>,
}

#[get("/donate")]
async fn list_donate(req: HttpRequest) -> Result<HttpResponse, Error> {
    let stripe_client = get_data::<StripeClient>(&req)?;
    let params = stripe::ListProducts::default();
    let products = stripe::Product::list(stripe_client, &params)
        .await
        .context("Failed to list products")?;
    let params = stripe::ListPrices::default();
    let prices = stripe::Price::list(stripe_client, &params)
        .await
        .context("Failed to list prices")?;
    Ok(HttpResponse::Ok().json(StripeProducts { products, prices }))
}
