use anyhow::Context;
use chrono::NaiveDateTime;
use oauth2::TokenResponse;
use reqwest::header;

use crate::{error::Error, serenity_models, AuthToken};

#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn base_url(&self) -> String {
        "https://discord.com/api/v10".to_string()
    }

    pub fn from_token(token: &AuthToken) -> Result<Self, Error> {
        let header_value = reqwest::header::HeaderValue::from_str(&format!(
            "Bearer {}",
            token.access_token().secret()
        ))
        .context("Failed to create header value")?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header_value);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self { inner: client })
    }

    pub fn from_bot_token(token: &str) -> Result<Self, Error> {
        let header_value = reqwest::header::HeaderValue::from_str(&format!("Bot {}", token))
            .context("Failed to create header value")?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header_value);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self { inner: client })
    }

    pub async fn fetch_current_user(&self) -> Result<serenity_models::CurrentUser, Error> {
        let url = format!("{}/users/@me", self.base_url());
        let resp = self.inner.get(&url).send().await?;
        if resp.status().is_success() {
            let user = resp.json::<serenity_models::CurrentUser>().await?;
            Ok(user)
        } else {
            Err(Error::CustomStatus(
                resp.status(),
                format!("{:?}", resp.bytes().await?),
            ))
        }
    }

    pub async fn fetch_current_user_guilds(
        &self,
    ) -> Result<Vec<serenity_models::GuildInfo>, Error> {
        let url = format!("{}/users/@me/guilds", self.base_url());
        let resp = self.inner.get(&url).send().await?;
        if resp.status().is_success() {
            let guilds = resp.json::<Vec<serenity_models::GuildInfo>>().await?;
            Ok(guilds)
        } else {
            Err(Error::CustomStatus(
                resp.status(),
                format!("{:?}", resp.bytes().await?),
            ))
        }
    }

    pub async fn fetch_guild_channels(
        &self,
        guild_id: &str,
    ) -> Result<Vec<serenity_models::Channel>, Error> {
        let url = format!("{}/guilds/{}/channels", self.base_url(), guild_id);
        let resp = self.inner.get(&url).send().await?;
        if resp.status().is_success() {
            let channels = resp.json::<Vec<serenity_models::Channel>>().await?;
            Ok(channels)
        } else {
            Err(Error::CustomStatus(
                resp.status(),
                format!("{:?}", resp.bytes().await?),
            ))
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_scheduled_non_external_event(
        &self,
        guild_id: &str,
        is_stage: bool,
        channel_id: &str,
        title: &str,
        description: Option<&str>,
        start_time: NaiveDateTime,
        end_time: Option<NaiveDateTime>,
        image: Option<&str>,
    ) -> Result<serenity_models::ScheduledEvent, Error> {
        let payload = CreateScheduledEventPayload {
            channel_id: Some(channel_id.to_string()),
            entity_metadata: None,
            name: title.to_string(),
            privacy_level: PrivacyLevel::GuildOnly,
            scheduled_start_time: start_time,
            scheduled_end_time: end_time,
            description: description.map(|d| d.to_string()),
            entity_type: if is_stage {
                EntityType::StageInstance
            } else {
                EntityType::Voice
            },
            image: image.map(|i| i.to_string()),
        };
        let url = format!("{}/guilds/{}/scheduled-events", self.base_url(), guild_id);
        let resp = self.inner.post(&url).json(&payload).send().await?;
        if resp.status().is_success() {
            let event = resp.json::<serenity_models::ScheduledEvent>().await?;
            Ok(event)
        } else {
            Err(Error::CustomStatus(
                resp.status(),
                format!("{:?}", resp.bytes().await?),
            ))
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_scheduled_external_event(
        &self,
        guild_id: &str,
        location: &str,
        title: &str,
        description: Option<&str>,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        image: Option<&str>,
    ) -> Result<serenity_models::ScheduledEvent, Error> {
        let payload = CreateScheduledEventPayload {
            channel_id: None,
            entity_metadata: Some(EntityMetaData {
                location: location.to_string(),
            }),
            name: title.to_string(),
            privacy_level: PrivacyLevel::GuildOnly,
            scheduled_start_time: start_time,
            scheduled_end_time: Some(end_time),
            description: description.map(|d| d.to_string()),
            entity_type: EntityType::External,
            image: image.map(|i| i.to_string()),
        };
        let url = format!("{}/guilds/{}/scheduled-events", self.base_url(), guild_id);
        let resp = self.inner.post(&url).json(&payload).send().await?;
        if resp.status().is_success() {
            let event = resp.json::<serenity_models::ScheduledEvent>().await?;
            Ok(event)
        } else {
            Err(Error::CustomStatus(
                resp.status(),
                format!("{:?}", resp.bytes().await?),
            ))
        }
    }

    pub async fn create_channel_invitation(
        &self,
        channel_id: &str,
        max_age: u32,
    ) -> Result<Invitation, Error> {
        let url = &format!("{}/channels/{}/invites", self.base_url(), channel_id);
        let inv_result = self
            .inner
            .post(url)
            .json(&serde_json::json!({
                "max_age": max_age,
            }))
            .send()
            .await?;
        if inv_result.status().is_success() {
            let inv = inv_result.json::<Invitation>().await?;
            Ok(inv)
        } else {
            Err(Error::CustomStatus(
                inv_result.status(),
                format!("{:?}", inv_result.bytes().await?),
            ))
        }
    }
    pub async fn post(
        &self,
        url: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, Error> {
        let url = format!("{}/{}", self.base_url(), url);
        let resp = self.inner.post(&url).json(body).send().await?;
        if resp.status().is_success() {
            let channels = resp.json::<serde_json::Value>().await?;
            Ok(channels)
        } else {
            Err(Error::CustomStatus(
                resp.status(),
                format!("{:?}", resp.bytes().await?),
            ))
        }
    }
}

impl AsRef<reqwest::Client> for Client {
    fn as_ref(&self) -> &reqwest::Client {
        &self.inner
    }
}

// ScheduledEvent Models

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateScheduledEventPayload {
    pub channel_id: Option<String>,
    pub entity_metadata: Option<EntityMetaData>,
    pub name: String,
    pub privacy_level: PrivacyLevel,
    pub scheduled_start_time: NaiveDateTime,
    pub scheduled_end_time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub entity_type: EntityType,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateScheduledEventPayload {
    pub channel_id: Option<String>,
    pub entity_metadata: Option<EntityMetaData>,
    pub name: String,
    pub privacy_level: PrivacyLevel,
    pub scheduled_start_time: NaiveDateTime,
    pub scheduled_end_time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub entity_type: EntityType,
    pub status: EventStatus,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntityMetaData {
    pub location: String,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum PrivacyLevel {
    GuildOnly = 2,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum EntityType {
    StageInstance = 1,
    Voice = 2,
    External = 3,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum EventStatus {
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Cancelled = 4,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invitation {
    pub code: String,
}
