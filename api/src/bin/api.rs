#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_web;

use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, App, HttpServer};
use anyhow::Context;
use api::routes;

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
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .context("Failed to connect to database")?;

    let secret_key = get_secret_key();

    let auth = api::auth_client::OAuth2Client::new(
        std::env::var("DISCORD_CLIENT_ID").unwrap(),
        std::env::var("DISCORD_CLIENT_SECRET").unwrap(),
        "https://discordapp.com/api/oauth2/authorize".to_string(),
        "https://discordapp.com/api/oauth2/token".to_string(),
        std::env::var("DISCORD_REDIRECT_URI").unwrap(),
        vec!["identify".to_string(), "guilds".to_string()],
    );

    let server = HttpServer::new(move || {
        App::new()
            .app_data(auth.clone())
            .app_data(pool.clone())
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
