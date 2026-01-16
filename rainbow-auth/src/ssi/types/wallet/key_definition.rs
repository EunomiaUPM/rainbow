/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyInfo {
    pub id: String
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
    pub keyset_handle: Option<Value>
}
