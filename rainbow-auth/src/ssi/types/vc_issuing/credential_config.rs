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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ssi::types::vc_issuing::VcType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialConfiguration {
    pub format: String,
    pub cryptographic_binding_methods_supported: Vec<String>,
    pub credential_signing_alg_values_supported: Vec<String>,
    pub credential_definition: CredentialDefinition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinition {
    pub r#type: Vec<String>,
}

impl CredentialConfiguration {
    pub fn basic() -> HashMap<String, CredentialConfiguration> {
        let mut credential_configurations_supported = HashMap::new();

        for vc_type in VcType::variants() {
            credential_configurations_supported.insert(
                vc_type.to_conf(),
                CredentialConfiguration {
                    format: "jwt_vc_json".to_string(),
                    cryptographic_binding_methods_supported: vec!["did".to_string()],
                    credential_signing_alg_values_supported: vec!["RSA".to_string()],
                    credential_definition: CredentialDefinition {
                        r#type: vec!["VerifiableCredential".to_string(), vc_type.to_string()],
                    },
                },
            );
        }

        credential_configurations_supported
    }
}
