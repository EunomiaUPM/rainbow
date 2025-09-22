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

mod wallet_trait;
pub use wallet_trait::RainbowSSIAuthWalletTrait;

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ClientConfig {
    pub class_id: String, // como se denomina una entidad a si misma
    pub cert_path: String,
    pub display: Option<DisplayInfo>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DisplayInfo {
    pub name: String,
    pub uri: Option<String>,
    pub logo_uri: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SSIWalletConfig {
    pub wallet_portal_url: String,
    pub wallet_portal_port: String,
    pub wallet_type: String,
    pub wallet_name: String,
    pub wallet_email: String,
    pub wallet_password: String,
    pub wallet_id: Option<String>,
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
    pub dids: Vec<DidsInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletSession {
    pub account_id: Option<String>,
    pub token: Option<String>,
    pub token_exp: Option<u64>,
    pub wallets: Vec<WalletInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyInfo {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyDefinition {
    pub algorithm: String,
    #[serde(rename = "cryptoProvider")]
    pub crypto_provider: String,
    #[serde(rename = "keyId")]
    pub key_id: KeyInfo,
    #[serde(rename = "keyPair")]
    pub key_pair: Value,
    #[serde(rename = "keyset_handle")]
    pub keyset_handle: Option<Value>,
}
