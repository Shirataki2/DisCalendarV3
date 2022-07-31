use anyhow::Context;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, url::Url, AuthUrl, ClientId, ClientSecret,
    CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope,
    StandardRevocableToken, TokenResponse, TokenUrl,
};

use crate::{prelude::*, AuthToken};

#[derive(Debug, Getters, Clone)]
pub struct OAuth2Client {
    #[get = "pub with_prefix"]
    client: BasicClient,
    #[get = "pub"]
    client_id: String,
    #[get = "pub"]
    client_secret: String,
    #[get = "pub"]
    auth_url: String,
    #[get = "pub"]
    token_url: String,
    #[get = "pub"]
    redirect_uri: String,
    #[get = "pub"]
    scopes: Vec<String>,
}

impl OAuth2Client {
    pub fn new<T>(
        client_id: T,
        client_secret: T,
        auth_url: T,
        token_url: T,
        redirect_uri: T,
        revoke_url: T,
        scopes: Vec<T>,
    ) -> OAuth2Client
    where
        T: Into<String>,
    {
        let client_id = client_id.into();
        let client_secret = client_secret.into();
        let auth_url = auth_url.into();
        let token_url = token_url.into();
        let redirect_uri = redirect_uri.into();
        let revoke_url = revoke_url.into();
        let scopes = scopes.into_iter().map(|s| s.into()).collect();

        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new(auth_url.clone()).expect("invalid auth url"),
            Some(TokenUrl::new(token_url.clone()).expect("invalid token url")),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.clone()).expect("invalid redirect uri"))
        .set_revocation_uri(RevocationUrl::new(revoke_url).expect("invalid revocation url"));
        OAuth2Client {
            client,
            client_id,
            client_secret,
            auth_url,
            token_url,
            redirect_uri,
            scopes,
        }
    }

    pub fn create_pkce_auth_url(&self, extra_params: &[(&str, &str)]) -> PkceAuth {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let mut auth = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_code_challenge)
            .add_scopes(
                self.scopes
                    .iter()
                    .map(|s| Scope::new(s.to_string()))
                    .collect::<Vec<_>>(),
            );
        for (name, value) in extra_params {
            auth = auth.add_extra_param(*name, *value);
        }
        let (auth_url, csrf_token) = auth.url();
        PkceAuth {
            auth_url,
            csrf_token,
            verifier: pkce_code_verifier,
        }
    }

    pub async fn revoke(&self, token: AuthToken) -> Result<(), Error> {
        let revoke_token: StandardRevocableToken = match token.refresh_token() {
            Some(token) => token.into(),
            None => token.access_token().into(),
        };

        self.client
            .revoke_token(revoke_token)
            .context("failed to revoke token")?
            .request_async(async_http_client)
            .await
            .context("failed to revoke token")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct PkceAuth {
    pub verifier: PkceCodeVerifier,
    pub auth_url: Url,
    pub csrf_token: CsrfToken,
}
