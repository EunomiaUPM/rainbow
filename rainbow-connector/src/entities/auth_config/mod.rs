use crate::entities::common::secret_management::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthenticationConfig {
    NoAuth,
    BasicAuth(BasicAuthConfig),
    BearerToken {
        token: SecretString,
    },
    ApiKey {
        key: String,
        value: SecretString,
        location: ApiKeyLocation, // Enum: Header, Query
    },
    OAuth2 {
        grant_type: OAuthGrantType, // Enum: ClientCredentials, AuthorizationCode...
        token_url: String,
        client_id: String,
        client_secret: SecretString,
        scopes: Vec<String>,
        // ... otros campos OAuth
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAuthConfig {
    username: String,
    password: SecretString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiKeyLocation {
    Header,
    Query,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OAuthGrantType {
    ClientCredentials,
    AuthorizationCode, /* ... */
}
