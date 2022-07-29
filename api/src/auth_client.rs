use oauth2::{
    basic::BasicClient, url::Url, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenUrl,
};

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
        let scopes = scopes.into_iter().map(|s| s.into()).collect();

        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new(auth_url.clone()).expect("invalid auth url"),
            Some(TokenUrl::new(token_url.clone()).expect("invalid token url")),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.clone()).expect("invalid redirect uri"));
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

    pub fn create_pkce_auth_url(&self) -> PkceAuth {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_code_challenge)
            .add_scopes(
                self.scopes
                    .iter()
                    .map(|s| Scope::new(s.to_string()))
                    .collect::<Vec<_>>(),
            )
            .url();
        PkceAuth {
            auth_url,
            csrf_token,
            verifier: pkce_code_verifier,
        }
    }
}

#[derive(Debug)]
pub struct PkceAuth {
    pub verifier: PkceCodeVerifier,
    pub auth_url: Url,
    pub csrf_token: CsrfToken,
}
