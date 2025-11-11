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

use super::CredentialConfiguration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct IssuerMetadata {
    pub issuer: String,
    pub credential_issuer: String,
    pub credential_endpoint: String,
    pub jwks_uri: String,
    pub credential_configurations_supported: HashMap<String, CredentialConfiguration>,
    pub authorization_servers: Vec<String>,
}

impl IssuerMetadata {
    pub fn new(host: &str) -> Self {
        let credential_configurations_supported = CredentialConfiguration::basic();

        IssuerMetadata {
            issuer: host.to_string(),
            credential_issuer: host.to_string(),
            credential_endpoint: format!("{}/credential", host),
            jwks_uri: format!("{}/jwks", host),
            credential_configurations_supported,
            authorization_servers: vec![host.to_string()],
        }
    }
}
