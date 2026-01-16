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

use crate::ssi::types::vc_issuing::VcType;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationNumber {
    pub id: String,
    pub r#type: String,
    #[serde(rename = "gx:registrationNumberType")]
    pub gx_registration_number_type: String,
    #[serde(rename = "gx:registrationNumberValue")]
    pub gx_registration_number_value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub r#type: String,
    pub gx_country: String,
    pub gx_locality: String,
    #[serde(rename = "gx:postalCode")]
    pub gx_postal_code: String,
    #[serde(rename = "gx:streetAddress")]
    pub gx_street_address: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalPersonRef {
    pub id: String,
    pub r#type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegalPersonCredentialSubject {
    pub id: String,
    pub r#type: String,
    #[serde(rename = "gx:registrationNumber")]
    pub gx_registration_number: RegistrationNumber,
    #[serde(rename = "gx:legalAddress")]
    pub gx_legal_address: Address,
    #[serde(rename = "gx:headquartersAddress")]
    pub gx_headquarters_address: Address,
    pub schema_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_description: Option<String>
}

impl LegalPersonCredentialSubject {
    pub fn default4gaia(kid: &str) -> LegalPersonCredentialSubject {
        let uid = uuid::Uuid::new_v4();
        LegalPersonCredentialSubject {
            id: uid.to_string(),
            r#type: VcType::LegalPerson.to_string(),
            gx_registration_number: RegistrationNumber {
                id: kid.to_string(),
                r#type: "gx:RegistrationNumber".to_string(),
                gx_registration_number_type: "gx:taxID".to_string(),
                gx_registration_number_value: "todo".to_string()
            },
            gx_legal_address: Address {
                id: None,
                r#type: "gx:Address".to_string(),
                gx_country: "ES".to_string(),
                gx_locality: "Madrid".to_string(),
                gx_postal_code: "28035".to_string(),
                gx_street_address: "Av. Complutense, 30, Moncloa - Aravaca, 28040 Madrid"
                    .to_string()
            },
            gx_headquarters_address: Address {
                id: None,
                r#type: "gx:Address".to_string(),
                gx_country: "ES".to_string(),
                gx_locality: "Madrid".to_string(),
                gx_postal_code: "28035".to_string(),
                gx_street_address: "Av. Complutense, 30, Moncloa - Aravaca, 28040 Madrid"
                    .to_string()
            },
            schema_name: "UPM to the sky".to_string(),
            schema_description: None
        }
    }
}
