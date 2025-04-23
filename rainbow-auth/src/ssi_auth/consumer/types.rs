/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */
use rainbow_common::config::config::get_consumer_client;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletLoginResponse {
    pub id: String,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthJwtclaims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub jti: String,
    pub iss: String,
    pub aud: String,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct WalletInfo {
    pub id: String,
    pub name: String,
    pub createdOn: String,
    pub addedOn: String,
    pub permission: String, // COMPLETAR
    pub dids: Option<Vec<Didsinfo>>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct Didsinfo {
    pub did: String,
    pub alias: String,
    pub document: String,
    pub keyId: String,
    pub default: bool,
    pub createdOn: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfoResponse {
    pub account: String,
    pub wallets: Vec<WalletInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReachProvider {
    pub id: String,
    pub url: String,
    pub actions: Vec<String>,
}

// ------------------------------------------------------------------------------------

#[derive(Deserialize, Serialize, Debug)]
pub struct VPexchange {
    pub did: String,
    pub presentationRequest: String,
    pub selectedCredentials: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MatchingVCs {
    pub addedOn: String,
    pub disclosures: String,
    pub document: String,
    pub format: String,
    pub id: String,
    pub parsedDocument: Value,
    pub pending: bool,
    pub wallet: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Permission {
    ADMINISTRATE, // ADD MORE
}

#[derive(Deserialize)]
pub struct Petition {
    pub access_token: Vec<Value>, // Required if requesting access token
    pub subject: Value,           // Required if requesting subject information
}
