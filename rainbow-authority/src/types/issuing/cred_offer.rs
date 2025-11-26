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

use crate::types::enums::vc_type::VcType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VCCredOffer {
    pub credential_issuer: String,
    pub grants: CredOfferGrants,
    pub credential_configuration_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredOfferGrants {
    #[serde(rename = "urn:ietf:params:oauth:grant-type:pre-authorized_code")]
    pub urn_pre_authorized_code: UrnPreAuthorizedCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrnPreAuthorizedCode {
    #[serde(rename = "pre-authorized_code")]
    pub pre_authorized_code: String,
}

impl VCCredOffer {
    pub fn new(issuer: String, token: String, vc_type: VcType) -> Self {
        VCCredOffer {
            credential_issuer: issuer,
            grants: CredOfferGrants { urn_pre_authorized_code: UrnPreAuthorizedCode { pre_authorized_code: token } },
            credential_configuration_ids: vec![vc_type.to_conf()],
        }
    }
}
