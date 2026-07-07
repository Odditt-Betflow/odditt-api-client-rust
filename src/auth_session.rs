//! Convenience authentication layer over the generated client (hand-written).
//!
//! Kept in .openapi-generator-ignore so regeneration preserves it.
//!
//! The API accepts two credential types, both exchanged for a short-lived Bearer
//! JWT at a login endpoint:
//!
//!   - an API key           -> POST /v1/auth/login  (sends the X-API-Key header)
//!   - client_id + secret   -> POST /v1/oauth/login (body {client_id, client_secret})
//!
//! Data endpoints also accept the API key directly via X-API-Key, so no login is
//! needed for them; the account endpoints (/v1/account/...) require the Bearer
//! token.
//!
//! [`AuthSession`] caches the token and refreshes it as needed. Call
//! [`AuthSession::configuration`] to obtain a ready-to-use [`Configuration`] with a
//! valid Bearer token (and the X-API-Key for an API-key session):
//!
//! ```ignore
//! use odditt_api_client::auth_session::AuthSession;
//! use odditt_api_client::apis::account_api;
//!
//! let session = AuthSession::from_api_key("YOUR_API_KEY");
//! let configuration = session.configuration().await?;
//! let keys = account_api::v1_account_api_keys_get(&configuration).await?;
//! ```

use std::sync::Mutex;

use chrono::{Duration, Utc};

use crate::apis::authentication_api;
use crate::apis::configuration::{ApiKey, Configuration};
use crate::models::{AuthOAuthLoginRequest, AuthRefreshRequest, AuthTokenPair};

/// An error obtaining or refreshing the Bearer token.
#[derive(Debug)]
pub struct AuthError(String);

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Odditt authentication failed: {}", self.0)
    }
}

impl std::error::Error for AuthError {}

struct TokenState {
    token_pair: Option<AuthTokenPair>,
    expires_at: Option<chrono::DateTime<Utc>>,
}

/// Auto-login/refresh authentication wrapper.
pub struct AuthSession {
    api_key: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    base_path: Option<String>,
    skew: Duration,
    state: Mutex<TokenState>,
}

impl AuthSession {
    /// Build a session from an API key.
    pub fn from_api_key(api_key: impl Into<String>) -> Self {
        Self::new(Some(api_key.into()), None, None)
    }

    /// Build a session from OAuth client credentials.
    pub fn from_client_credentials(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        Self::new(None, Some(client_id.into()), Some(client_secret.into()))
    }

    fn new(api_key: Option<String>, client_id: Option<String>, client_secret: Option<String>) -> Self {
        AuthSession {
            api_key,
            client_id,
            client_secret,
            base_path: None,
            skew: Duration::seconds(60),
            state: Mutex::new(TokenState {
                token_pair: None,
                expires_at: None,
            }),
        }
    }

    /// Override the API base URL.
    pub fn with_base_path(mut self, base_path: impl Into<String>) -> Self {
        self.base_path = Some(base_path.into());
        self
    }

    /// Refresh the token this many seconds before it expires (default 60).
    pub fn with_skew_seconds(mut self, seconds: i64) -> Self {
        self.skew = Duration::seconds(seconds);
        self
    }

    /// Returns a [`Configuration`] with a valid Bearer token (and X-API-Key for an
    /// API-key session), logging in or refreshing as needed.
    pub async fn configuration(&self) -> Result<Configuration, AuthError> {
        let token = self.bearer_token().await?;
        let mut configuration = Configuration::new();
        if let Some(base_path) = &self.base_path {
            configuration.base_path = base_path.clone();
        }
        if let Some(api_key) = &self.api_key {
            configuration.api_key = Some(ApiKey {
                prefix: None,
                key: api_key.clone(),
            });
        }
        configuration.bearer_access_token = Some(token);
        Ok(configuration)
    }

    async fn bearer_token(&self) -> Result<String, AuthError> {
        // Fast path: a still-valid cached token. The lock is released before any
        // await below.
        {
            let state = self.state.lock().unwrap();
            if let (Some(pair), Some(expiry)) = (&state.token_pair, state.expires_at) {
                if Utc::now() < expiry - self.skew {
                    return Ok(pair.access_token.clone());
                }
            }
        }

        let refresh_token = {
            let state = self.state.lock().unwrap();
            state.token_pair.as_ref().map(|pair| pair.refresh_token.clone())
        };

        let pair = match refresh_token {
            Some(token) => self.refresh(token).await?,
            None => self.login().await?,
        };

        let expiry = pair.expires_at.with_timezone(&Utc);
        let access_token = pair.access_token.clone();

        let mut state = self.state.lock().unwrap();
        state.token_pair = Some(pair);
        state.expires_at = Some(expiry);
        Ok(access_token)
    }

    fn auth_config(&self) -> Configuration {
        let mut configuration = Configuration::new();
        if let Some(base_path) = &self.base_path {
            configuration.base_path = base_path.clone();
        }
        configuration
    }

    async fn login(&self) -> Result<AuthTokenPair, AuthError> {
        let configuration = self.auth_config();
        if let Some(api_key) = &self.api_key {
            authentication_api::v1_auth_login_post(&configuration, api_key)
                .await
                .map_err(|e| AuthError(format!("{e:?}")))
        } else {
            let request = AuthOAuthLoginRequest::new(
                self.client_id.clone().unwrap_or_default(),
                self.client_secret.clone().unwrap_or_default(),
            );
            authentication_api::v1_oauth_login_post(&configuration, request)
                .await
                .map_err(|e| AuthError(format!("{e:?}")))
        }
    }

    async fn refresh(&self, refresh_token: String) -> Result<AuthTokenPair, AuthError> {
        let configuration = self.auth_config();
        let request = AuthRefreshRequest::new(refresh_token);
        let result = if self.api_key.is_some() {
            authentication_api::v1_auth_refresh_post(&configuration, request).await
        } else {
            authentication_api::v1_oauth_refresh_post(&configuration, request).await
        };
        match result {
            Ok(pair) => Ok(pair),
            Err(_) => self.login().await, // a rejected refresh token falls back to a full login
        }
    }
}
