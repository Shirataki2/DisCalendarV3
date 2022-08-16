use crate::{
    models::{event, prelude::*},
    prelude::*,
};
use chrono::NaiveDateTime as DateTime;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[repr(u8)]
pub enum DiscordEventKind {
    #[serde(rename = "stage_channel")]
    StageChannel = 1,
    #[serde(rename = "voice_channel")]
    VoiceChannel = 2,
    #[serde(rename = "somewhere_else")]
    SomewhereElse = 3,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscordEventPayload {
    pub kind: DiscordEventKind,
    pub place: String,
    pub cover: Option<String>,
    #[serde(default)]
    pub create_invitation: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventCreateBody {
    pub name: String,
    pub description: Option<String>,
    pub notifications: String,
    pub color: String,
    pub is_all_day: bool,
    pub start_at: DateTime,
    pub end_at: DateTime,
    pub notify_channel_id: Option<String>,
    pub discord_event: Option<DiscordEventPayload>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventCreateResponse {
    pub id: i32,
    pub invitation_url: Option<String>,
}

#[post("")]
async fn create(
    req: HttpRequest,
    id: web::Path<u64>,
    body: web::Json<EventCreateBody>,
) -> Result<HttpResponse, Error> {
    let event_name = &body.name;
    let guild_id = id.into_inner();

    let (invitation_url, scheduled_event) = if let Some(ref event) = body.discord_event {
        if event.create_invitation && event.kind == DiscordEventKind::SomewhereElse {
            return Err(Error::new(
                StatusCode::BAD_REQUEST,
                "Somewhere Else event cannot create invitation",
            ));
        }
        let AppData {
            discord_bot_token, ..
        } = get_data::<AppData>(&req)?;
        let bot = Client::from_bot_token(discord_bot_token)?;
        let scheduled_event = if event.kind != DiscordEventKind::SomewhereElse {
            bot.create_scheduled_non_external_event(
                &guild_id.to_string(),
                event.kind == DiscordEventKind::StageChannel,
                &event.place,
                event_name,
                body.description.as_deref(),
                body.start_at,
                Some(body.end_at),
                event.cover.as_deref(),
            )
            .await?
        } else {
            bot.create_scheduled_external_event(
                &guild_id.to_string(),
                &event.place,
                event_name,
                body.description.as_deref(),
                body.start_at,
                body.end_at,
                event.cover.as_deref(),
            )
            .await?
        };

        if event.create_invitation {
            let channel_id = &event.place;
            let invitation = bot.create_channel_invitation(channel_id, 604800).await?;
            let invite_url = format!(
                "https://discord.gg/{}?event={}",
                invitation.code, scheduled_event.id
            );
            info!("Created invitation: {}", invite_url);
            (Some(invite_url), Some(scheduled_event))
        } else {
            (None, Some(scheduled_event))
        }
    } else {
        (None, None)
    };

    let event = event::ActiveModel {
        name: Set(event_name.to_string()),
        description: Set(body.description.clone()),
        color: Set(body.color.clone()),
        is_all_day: Set(body.is_all_day),
        start_at: Set(body.start_at),
        end_at: Set(body.end_at),
        notifications: Set(body.notifications.clone()),
        guild_id: Set(guild_id.to_string()),
        discord_event_id: Set(scheduled_event.map(|e| e.id.to_string())),
        notify_channel_id: Set(body.notify_channel_id.clone()),
        ..Default::default()
    };

    let db = get_data::<DatabaseConnection>(&req)?;
    let result = Event::insert(event).exec(db).await?;

    let id = result.last_insert_id;
    Ok(HttpResponse::Ok().json(EventCreateResponse { id, invitation_url }))
}
