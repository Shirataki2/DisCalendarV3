#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
#[macro_use]
extern crate getset;

use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse};
pub mod serenity_models {
    pub use serenity::model::user::CurrentUser;
}

pub type AuthToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub mod auth_client;
pub mod discord;
pub mod error;
pub mod prelude;
pub mod routes;
