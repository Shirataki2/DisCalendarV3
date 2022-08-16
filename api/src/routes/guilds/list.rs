use crate::prelude::*;
use crate::{
    models::{guild, prelude::*},
    routes::get_data,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerListResponse {
    invited: Vec<serenity_models::GuildInfo>,
    invitable: Vec<serenity_models::GuildInfo>,
    others: Vec<serenity_models::GuildInfo>,
}

#[get("/list")]
pub async fn guild_list(sess: Session, req: HttpRequest) -> Result<HttpResponse, Error> {
    // Authenticate the user
    let auth_token = match sess.get::<crate::AuthToken>("token")? {
        Some(auth_token) => auth_token,
        None => return Err(Error::Unauthorized("No auth token found".into())),
    };
    let client = Client::from_token(&auth_token)?;
    let conn = get_data::<DatabaseConnection>(&req)?;

    // Get the user's guilds
    let guilds = client.fetch_current_user_guilds().await?;
    let guild_ids = guilds
        .iter()
        .map(|g| g.id.0.to_string())
        .collect::<Vec<_>>();

    sess.insert("guild_ids", guild_ids.clone())?;
    // Get the invited guilds from the database
    let invited_guilds = Guild::find()
        .filter(guild::Column::GuildId.is_in(guild_ids))
        .all(conn)
        .await?;

    let (mut invited, mut invitable, mut others) = (Vec::new(), Vec::new(), Vec::new());
    for guild in guilds {
        if invited_guilds
            .iter()
            .map(|g| g.guild_id.to_string())
            .any(|x| x == guild.id.0.to_string())
        {
            invited.push(guild);
        } else if is_invitable(&guild) {
            invitable.push(guild);
        } else {
            others.push(guild);
        }
    }

    Ok(HttpResponse::Ok().json(ServerListResponse {
        invited,
        invitable,
        others,
    }))
}

fn is_invitable(guild: &serenity_models::GuildInfo) -> bool {
    if guild.owner {
        return true;
    }
    let permissions = guild.permissions.parse::<u64>();
    if permissions.is_err() {
        return false;
    }
    let permissions = permissions.unwrap();
    info!("{:?}", permissions);
    let permissions = serenity_models::Permissions::from_bits(permissions).unwrap_or_default();
    permissions.administrator() || permissions.manage_guild()
}
