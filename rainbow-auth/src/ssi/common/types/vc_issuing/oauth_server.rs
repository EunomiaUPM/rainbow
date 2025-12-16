/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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
pub struct AuthServerMetadata {
    pub issuer: String,
    pub credential_issuer: String,
    pub credential_endpoint: String,
    pub authorization_endpoint: String,
    pub pushed_authorization_request_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
    pub batch_credential_endpoint: String,
    pub deferred_credential_endpoint: String,
    pub scopes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub response_modes_supported: Vec<String>,
    pub grant_types_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub code_challenge_methods_supported: Vec<String>,
    pub credential_configurations_supported: HashMap<String, CredentialConfiguration>,
    pub authorization_servers: Vec<String>,
}

impl AuthServerMetadata {
    pub fn new(host: &str) -> Self {
        let credential_configurations_supported = CredentialConfiguration::basic();

        AuthServerMetadata {
            issuer: host.to_string(),
            credential_issuer: host.to_string(),
            credential_endpoint: format!("{}/credential", host),
            authorization_endpoint: format!("{}/authorize", host),
            pushed_authorization_request_endpoint: format!("{}/par", host),
            token_endpoint: format!("{}/token", host),
            jwks_uri: format!("{}/jwks", host),
            batch_credential_endpoint: format!("{}/credential", host),
            deferred_credential_endpoint: format!("{}/credential_deferred", host),
            scopes_supported: vec!["openid".to_string()],
            response_types_supported: vec!["code".to_string(), "vp_token".to_string(), "id_token".to_string()],
            response_modes_supported: vec!["query".to_string(), "fragment".to_string()],
            grant_types_supported: vec![
                "authorization_code".to_string(),
                "urn:ietf:params:oauth:grant-type:pre-authorized_code".to_string(),
            ],
            subject_types_supported: vec!["public".to_string()],
            id_token_signing_alg_values_supported: vec!["RSA".to_string()],
            code_challenge_methods_supported: vec!["S256".to_string()],
            credential_configurations_supported,
            authorization_servers: vec![host.to_string()],
        }
    }
}
