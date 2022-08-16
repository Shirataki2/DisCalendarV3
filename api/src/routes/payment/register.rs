use crate::models::{billing, prelude::*};
use crate::prelude::*;
use sea_orm::Set;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterQuery {
    pub session_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub customer_id: String,
}

#[post("/register")]
async fn register(
    sess: Session,
    req: HttpRequest,
    query: web::Query<RegisterQuery>,
) -> Result<HttpResponse, Error> {
    let user = match sess.get::<serenity_models::CurrentUser>("user")? {
        Some(user) => user,
        None => {
            return Err(Error::Unauthorized("User Not Found".to_string()));
        }
    };
    let conn = get_data::<DatabaseConnection>(&req)?;
    info!("Register started: User:{:?}", user.name);
    let stripe_client = get_data::<StripeClient>(&req)?;

    let url = format!("/checkout/sessions/{}", query.session_id);
    let checkout_session = match stripe_client.get::<stripe::CheckoutSession>(&url).await {
        Ok(checkout_session) => checkout_session,
        Err(err) => {
            return Err(Error::CustomStatus(
                StatusCode::BAD_REQUEST,
                format!("Invalid Session ID: {}", err),
            ));
        }
    };
    let customer = match checkout_session.customer.as_ref() {
        Some(customer) => customer,
        None => {
            return Err(Error::CustomStatus(
                StatusCode::BAD_REQUEST,
                "Customer Not Found".to_string(),
            ));
        }
    };
    let customer_id = customer.id().to_string();

    // Check current user has a customer ID
    let db_result = Billing::find()
        .filter(billing::Column::UserId.eq(user.id.to_string()))
        .one(conn)
        .await;
    match db_result {
        Ok(None) => {
            let model = billing::ActiveModel {
                user_id: Set(user.id.to_string()),
                customer_id: Set(Some(customer_id.clone())),
            };
            let result = model.insert(conn).await;
            match result {
                Ok(_) => {
                    info!("User:{:?} registered", user.name);
                    Ok(HttpResponse::Ok().json(RegisterResponse { customer_id }))
                }
                Err(err) => {
                    error!("Error registering user:{:?} {:?}", user.name, err);
                    Err(Error::CustomStatus(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error Registering User".to_string(),
                    ))
                }
            }
        }
        Ok(Some(bill)) => match bill.customer_id.as_ref() {
            Some(db_customer_id) => {
                if &customer_id == db_customer_id {
                    info!("User:{:?} already registered", user.name);
                    Ok(HttpResponse::Ok().json(RegisterResponse { customer_id }))
                } else {
                    error!(
                        "User:{:?} already registered with different customer ID",
                        user.name
                    );
                    Err(Error::CustomStatus(
                        StatusCode::BAD_REQUEST,
                        "User Already Registered".to_string(),
                    ))
                }
            }
            None => {
                let mut bill: billing::ActiveModel = bill.into();
                bill.customer_id = Set(Some(customer_id.clone()));
                let result = bill.update(conn).await;
                match result {
                    Ok(_) => {
                        info!("User:{:?} registered", user.name);
                        Ok(HttpResponse::Ok().json(RegisterResponse { customer_id }))
                    }
                    Err(err) => {
                        error!("Error registering user:{:?} {:?}", user.name, err);
                        Err(Error::CustomStatus(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Error Registering User".to_string(),
                        ))
                    }
                }
            }
        },
        Err(err) => {
            return Err(Error::CustomStatus(
                StatusCode::BAD_REQUEST,
                format!("Database Error: {}", err),
            ));
        }
    }
}
