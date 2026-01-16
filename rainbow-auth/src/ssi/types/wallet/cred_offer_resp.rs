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

#[derive(Serialize, Deserialize, Debug)]
pub struct CredentialOfferResponse {
    pub credential_configuration_ids: Vec<String>,
    pub credential_issuer: String,
    pub grants: Grants
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grants {
    #[serde(rename = "urn:ietf:params:oauth:grant-type:pre-authorized_code")]
    pub pre_authorized_code: PreAuthorizedGrant
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreAuthorizedGrant {
    #[serde(rename = "pre-authorized_code")]
    pub pre_authorized_code: String
}
