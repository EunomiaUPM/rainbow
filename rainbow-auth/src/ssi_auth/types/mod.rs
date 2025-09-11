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

use rainbow_common::ssi_wallet::WalletInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletInfoResponse {
    pub account: String,
    pub wallets: Vec<WalletInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReachProvider {
    pub id: String,
    pub slug: String,
    pub url: String,
    pub actions: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReachAuthority {
    pub id: String,
    pub slug: String,
    pub url: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct RedirectResponse {
    #[serde(rename = "redirectUri")]
    pub redirect_uri: String,
}

#[derive(Deserialize, Serialize)]
pub struct CallbackResponse {
    pub hash: String,
    pub interact_ref: String,
}

#[derive(Deserialize, Serialize)]
pub struct Url2RequestVC {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct RefBody {
    pub interact_ref: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub jti: String,
    pub sub: String, // Optional. Subject (whom token refers to)
    pub iss: String, // Optional. Issuer
    pub aud: String, // Optional. Audience
    pub scopes: String,
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub nbf: usize, // Optional. not before
}

impl Claims {
    pub fn new(sub: String, iss: String, aud: String, scopes: String, exp: usize) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        let nbf: usize = now.as_secs() as usize;
        let jti = Uuid::new_v4().to_string();

        Self { jti, sub, iss, aud, scopes, exp, nbf }
    }
}

pub fn trim_4_base(input: &str) -> String {
    let slashes: Vec<usize> = input.match_indices('/').map(|(i, _)| i).collect();

    if slashes.len() < 3 {
        return input.to_string();
    }

    let cut_index = slashes[2];

    input[..cut_index].to_string()
}

