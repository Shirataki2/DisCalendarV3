use crate::{prelude::*, routes::get_data};

#[get("/channels")]
pub async fn channels(
    sess: Session,
    req: HttpRequest,
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // Authenticate the user
    let auth_token = match sess.get::<crate::AuthToken>("token")? {
        Some(auth_token) => auth_token,
        None => return Err(Error::Unauthorized("No auth token found".into())),
    };
    let channel_id = path.into_inner().to_string();

    let mut user_guilds = match sess.get::<Vec<String>>("guild_ids")? {
        Some(guild_ids) => guild_ids,
        None => vec![],
    };
    if user_guilds.is_empty() {
        let client = Client::from_token(&auth_token)?;

        // Get the user's guilds
        let guilds = client.fetch_current_user_guilds().await?;
        let guild_ids = guilds
            .iter()
            .map(|g| g.id.0.to_string())
            .collect::<Vec<_>>();
        sess.insert("guild_ids", guild_ids.clone())?;
        user_guilds = guild_ids;
    }

    if !user_guilds.contains(&channel_id) {
        return Err(Error::Forbidden);
    }

    let AppData { discord_bot_token } = get_data::<AppData>(&req)?;
    let bot = Client::from_bot_token(discord_bot_token)?;

    let channels = bot.fetch_guild_channels(&channel_id).await?;

    Ok(HttpResponse::Ok().json(channels))
}
