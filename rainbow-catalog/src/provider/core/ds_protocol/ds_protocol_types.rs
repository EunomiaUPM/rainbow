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

use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::context_field::ContextField;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::protocol::ProtocolValidate;
use rainbow_common::utils::get_urn;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogRequest {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    pub filter: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogResponse {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@id")]
    pub id: Urn,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "participantId")]
    pub participant_id: String,
    pub dataset: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<Vec<DatasetResponse>>,
    pub service: Vec<ServiceResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetResponse {
    #[serde(rename = "@id")]
    pub id: Urn,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "hasPolicy")]
    pub has_policy: Vec<OdrlOffer>,
    #[serde(rename = "distribution")]
    pub distribution: Vec<DistributionResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionResponse {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "format")]
    pub format: DctFormats,
    #[serde(rename = "accessService")]
    pub access_service: Urn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceResponse {
    #[serde(rename = "@id")]
    pub id: Urn,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "endpointURL")]
    pub endpoint_url: String,
}

impl Default for CatalogResponse {
    fn default() -> Self {
        Self {
            context: ContextField::default(),
            participant_id: "".to_string(),
            _type: EntityTypes::Catalog.to_string(),
            id: get_urn(None),
            dataset: vec![],
            catalog: None,
            service: vec![],
        }
    }
}

impl ProtocolValidate for CatalogResponse {
    fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogError {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

impl Default for CatalogError {
    fn default() -> Self {
        Self { context: ContextField::default(), _type: "CatalogError".to_string(), code: None, reason: None }
    }
}
