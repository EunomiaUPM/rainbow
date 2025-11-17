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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialSubject4DataSpace {
    pub id: String,
    pub r#type: String,
    #[serde(rename = "DataspaceId")]
    pub dataspace_id: String,
    #[serde(rename = "LegalName")]
    pub legal_name: String,
}

impl CredentialSubject4DataSpace {
    pub fn new(id: String, legal_name: String) -> Self {
        CredentialSubject4DataSpace {
            id,
            r#type: "DataspaceParticipant".to_string(),
            dataspace_id: "RainbowDataSpace".to_string(),
            legal_name,
        }
    }
}
