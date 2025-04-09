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

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct GrantPetition {
    pub access_token: AccessTokenRequirements4GP,
    pub subject: Option<Subject4GP>, // REQUIRED if requesting subject information
    pub client: String,
    pub user: Option<String>,
    pub interact: Interact4GP,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenRequirements4GP {
    pub access: Access4AT,
    pub label: Option<String>, // REQUIRED if used as part of a request for multiple access tokens
    pub flags: Option<String>, // A set of flags that indicate desired attributes or behavior to be attached to the access token by the AS
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Access4AT {
    _type: String,
    actions: Option<Vec<Actions4Access4AT>>,
    locations: Option<Vec<String>>,
    datatypes: Option<Vec<String>>,
    identifier: Option<String>,
    privileges: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Actions4Access4AT {
    TALK,
    NEGOTIATE,
    EXCHANGE,
    READ,
    WRITE,
    DELETE,
}

impl Actions4Access4AT {
    pub fn as_str(&self) -> &str {
        match *self {
            Actions4Access4AT::TALK => "talk",
            Actions4Access4AT::NEGOTIATE => "negotiate",
            Actions4Access4AT::EXCHANGE => "exchange",
            Actions4Access4AT::READ => "read",
            Actions4Access4AT::WRITE => "write",
            Actions4Access4AT::DELETE => "delete",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject4GP {
    sub_id_formats: Option<Vec<String>>, // REQUIRED if Subject Identifiers are requested
    assertion_formats: Option<Vec<String>>, // REQUIRED if assertions are requested
    sub_ids: Option<Value>, // If omited assume that subject information requests are about the current user
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interact4GP {
    start: Vec<String>,
    finish: Finish4Interact, // REQUIRED because DataSpace Protocl is based on requirements
    hints: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Finish4Interact {
    method: String,
    uri: Option<String>, // REQUIRED for redirect and push methods
    nonce: String,
    hash_method: Option<String>,
}





































pub struct COMPLETAR {}


// ------------------------------------------------------------------------------------



#[derive(Deserialize)]
pub struct WalletLoginResponse {
    pub id: String,
    pub username: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct WalletInfoResponse {
    pub account: String,
    pub wallets: Vec<WalletInfo>,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct WalletInfo {
    pub id: String,
    pub name: String,
    pub createdOn: String,
    pub addedOn: String,
    pub permission: String, // COMPLETAR
    pub dids: Option<Vec<Didsinfo>>,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Didsinfo {
    pub did: String,
    pub alias: String,
    pub document: String,
    pub keyId: String,
    pub default: bool,
    pub createdOn: String,
}

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
    pub subject: Value,           // Required if requesting subject information
}
