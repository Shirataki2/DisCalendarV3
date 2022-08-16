#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_web;

use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, App, HttpServer};
use anyhow::Context;
use api::{prelude::*, routes};

#[get("/")]
async fn index() -> String {
    String::from("DisCalendar API v3.0!")
}

fn get_secret_key() -> Key {
    let key = std::env::var("SECRET_KEY").unwrap();
    let key = base64::decode(&key).unwrap();
    Key::from(&key)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .unwrap();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sea_orm::Database::connect(&database_url)
        .await
        .context("Failed to connect to database")?;

    let secret_key = get_secret_key();

    let app_secret = yup_oauth2::read_application_secret("credentials.json")
        .await
        .expect("Failed to read application secret");
    let google_auth = Box::new(api::auth_client::OAuth2Client::new(
        app_secret.client_id.to_string(),
        app_secret.client_secret.to_string(),
        "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
        "https://www.googleapis.com/oauth2/v3/token".to_string(),
        "http://localhost:15000/oauth2/google/callback".to_string(),
        "https://oauth2.googleapis.com/revoke".to_string(),
        vec!["https://www.googleapis.com/auth/calendar".to_string()],
    ));

    let discord_auth = api::auth_client::OAuth2Client::new(
        std::env::var("DISCORD_CLIENT_ID").unwrap(),
        std::env::var("DISCORD_CLIENT_SECRET").unwrap(),
        "https://discordapp.com/api/oauth2/authorize".to_string(),
        "https://discordapp.com/api/oauth2/token".to_string(),
        std::env::var("DISCORD_REDIRECT_URI").unwrap(),
        "https://discord.com/api/oauth2/token/revoke".to_string(),
        vec!["identify".to_string(), "guilds".to_string()],
    );

    let api_base_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:15000".to_string());

    let front_base_url =
        std::env::var("FRONT_BASE_URL").unwrap_or_else(|_| "http://localhost:16655".to_string());

    let discord_bot_token = std::env::var("DISCORD_BOT_TOKEN").unwrap();

    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").unwrap();

    let stripe_client = stripe::Client::new(&stripe_secret_key);

    let app_data = AppData {
        api_base_url,
        front_base_url,
        discord_bot_token,
        stripe_secret_key,
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(google_auth.clone())
            .app_data(discord_auth.clone())
            .app_data(pool.clone())
            .app_data(app_secret.clone())
            .app_data(app_data.clone())
            .app_data(stripe_client.clone())
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new("redis:6379"),
                secret_key.clone(),
            ))
            .service(index)
            .configure(routes::init_routes)
    })
    .bind((host.as_str(), port))?;

    info!("Listening on http://{}:{}", host, port);

    server.run().await?;

    Ok(())
}
