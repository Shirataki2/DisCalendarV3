#[macro_use]
extern crate log;
use api::models::guild;
use api::models::prelude::*;
use sea_orm::entity::*;

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = sea_orm::Database::connect(&database_url).await.unwrap();
    let guild = guild::ActiveModel {
        guild_id: Set("2000".into()),
        name: Set("Test Guild".into()),
        avatar_url: Set(Some(
            "https://discordapp.com/assets/322c936a8c8be1b803cd94861bdfa868.png".into(),
        )),
        locale: Set("en-US".into()),
        ..Default::default()
    };
    let guild = guild.save(&conn).await.unwrap();
    let guild_id = *guild.id.as_ref();

    info!("{:?}", guild);

    pause();
    let guild: guild::ActiveModel = Guild::find_by_id(guild_id)
        .one(&conn)
        .await
        .unwrap()
        .unwrap()
        .into();
    guild.delete(&conn).await.unwrap();
    Ok(())
}
