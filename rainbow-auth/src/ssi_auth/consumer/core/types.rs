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
use rainbow_common::ssi_wallet::WalletInfo;

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
    // pub id: String,
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
