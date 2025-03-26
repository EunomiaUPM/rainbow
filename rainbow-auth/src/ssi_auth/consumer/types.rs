use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct WalletLoginResponse {
    pub id: String,
    pub username: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct WalletInfo {
    pub id: String,
    pub name: String,
    pub created_on: String,
    pub added_on: String,
    pub permission: Permission,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Permission {
    ADMINISTRATE, // ADD MORE
}

#[derive(Deserialize)]
pub struct Jwtclaims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub jti: String,
    pub iss: String,
    pub aud: String,
}

#[derive(Deserialize)]
pub struct Petition {
    pub access_token: Vec<Value>, // Required if requesting access token
    pub subject: Value, // Required if requesting subject information

}