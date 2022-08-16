#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde_repr;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
#[macro_use]
extern crate getset;

use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse};
pub mod serenity_models {
    pub use serenity::model::{
        channel::Channel,
        guild::{ScheduledEvent, ScheduledEventMetadata, ScheduledEventType},
        id::GuildId,
        user::CurrentUser,
        Permissions,
    };

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GuildInfo {
        pub id: GuildId,
        pub icon: Option<String>,
        pub name: String,
        pub owner: bool,
        pub permissions: String,
    }
}

pub type AuthToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub mod auth_client;
pub mod discord;
pub mod error;
pub mod models;
pub mod prelude;
pub mod routes;

#[derive(Debug, Clone)]
pub struct AppData {
    pub api_base_url: String,
    pub front_base_url: String,
    pub discord_bot_token: String,
    pub stripe_secret_key: String,
}
