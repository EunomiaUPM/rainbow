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

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletLoginResponse {
    pub id: String,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthJwtClaims {
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
    #[serde(rename = "createdOn")]
    pub created_on: String,
    #[serde(rename = "addedOn")]
    pub added_on: String,
    pub permission: String, // TODO
    pub dids: Option<Vec<DidsInfo>>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct DidsInfo {
    pub did: String,
    pub alias: String,
    pub document: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub default: bool,
    #[serde(rename = "createdOn")]
    pub created_on: String,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct MatchingVCs {
    #[serde(rename = "addedOn")]
    pub added_on: String,
    pub disclosures: String,
    pub document: String,
    pub format: String,
    pub id: String,
    #[serde(rename = "parsedDocument")]
    pub parsed_document: Value,
    pub pending: bool,
    pub wallet: String,
}