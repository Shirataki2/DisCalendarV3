use anyhow::Context;
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
}

impl AsRef<reqwest::Client> for Client {
    fn as_ref(&self) -> &reqwest::Client {
        &self.inner
    }
}
