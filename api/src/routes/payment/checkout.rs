use std::str::FromStr;

use stripe::CheckoutSession;

use crate::{
    models::{billing, prelude::*},
    prelude::*,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckoutQuery {
    pub price_id: String,
}

#[get("/checkout")]
async fn checkout(
    query: web::Query<CheckoutQuery>,
    sess: Session,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let user = match sess.get::<serenity_models::CurrentUser>("user")? {
        Some(user) => user,
        None => {
            return Err(Error::Unauthorized("User Not Found".to_string()));
        }
    };
    info!("Checkout started: User:{:?}", user.name);
    let stripe_client = get_data::<StripeClient>(&req)?;
    let price_id = match stripe::PriceId::from_str(&query.price_id) {
        Ok(price_id) => price_id,
        Err(err) => {
            return Err(Error::CustomStatus(
                StatusCode::BAD_REQUEST,
                format!("Invalid Price ID: {}", err),
            ));
        }
    };

    let price = match stripe::Price::retrieve(stripe_client, &price_id, &[]).await {
        Ok(price) => price,
        Err(err) => {
            return Err(Error::CustomStatus(
                StatusCode::BAD_REQUEST,
                format!("Invalid Price ID: {}", err),
            ));
        }
    };

    let base_url = &get_data::<AppData>(&req)?.front_base_url;
    let success_url = format!(
        "{}/payment?status=success&session_id={{CHECKOUT_SESSION_ID}}",
        &base_url
    );
    let cancel_url = format!("{}/payment?status=cancel", &base_url);

    let mode = match price.recurring {
        Some(_) => stripe::CheckoutSessionMode::Subscription,
        None => stripe::CheckoutSessionMode::Payment,
    };

    let customer = {
        let db_result = Billing::find()
            .filter(billing::Column::UserId.eq(user.id.to_string()))
            .one(get_data::<DatabaseConnection>(&req)?)
            .await;
        match db_result {
            Ok(Some(billing)) => billing
                .customer_id
                .map(|id| stripe::CustomerId::from_str(&id).unwrap_or_default()),
            Ok(None) => None,
            Err(err) => {
                return Err(Error::CustomStatus(
                    StatusCode::BAD_REQUEST,
                    format!("Invalid Customer ID: {}", err),
                ));
            }
        }
    };

    let mut params = stripe::CreateCheckoutSession::new(&cancel_url, &success_url);
    params.mode = Some(mode);
    params.customer = customer;
    params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
        price: Some(price_id.to_string()),
        quantity: Some(1),
        ..Default::default()
    }]);

    let session_url = match stripe::CheckoutSession::create(stripe_client, params).await {
        Ok(CheckoutSession { url, .. }) if url.is_some() => url.unwrap(),
        Err(err) => {
            error!("Error creating checkout session: {:?}", err);
            return Err(Error::CustomStatus(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error creating checkout session: {}", err),
            ));
        }
        _ => {
            return Err(Error::CustomStatus(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create checkout session url".to_string(),
            ));
        }
    };
    Ok(HttpResponse::Ok().body(session_url))
}
