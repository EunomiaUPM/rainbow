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

#[derive(Deserialize, Serialize, Debug)]
pub struct MatchingVCs {
    #[serde(rename = "addedOn")]
    pub added_on: String,
    pub disclosures: String,
    pub document: String,
    pub format: String,
    pub id: String,
    #[serde(rename = "parsedDocument")]
    pub parsed_document: Value,
    pub pending: bool,
    pub wallet: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchVCsRequest {
    pub did: String,
    #[serde(rename = "presentationRequest")]
    pub presentation_request: String,
    #[serde(rename = "selectedCredentials")]
    pub selected_credentials: Vec<String>
}
