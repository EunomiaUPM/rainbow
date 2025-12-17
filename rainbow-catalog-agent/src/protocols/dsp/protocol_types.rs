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

use rainbow_common::protocol::context_field::ContextField;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CatalogMessageWrapper<T>
where
    T: CatalogMessageTrait,
{
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: CatalogMessageType,
    #[serde(flatten)]
    pub dto: T,
}

pub trait CatalogMessageTrait: Debug + Send + Sync {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CatalogRequestMessageDto {
    pub filter: serde_json::Value,
}

impl CatalogMessageTrait for CatalogRequestMessageDto {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CatalogMessageDto {
    #[serde(rename = "@id")]
    pub id: Urn,
    pub participant_id: String,
    pub catalog: String,
    pub dataset: String,
    pub distribution: String,
    pub service: Urn,
}

impl CatalogMessageTrait for CatalogMessageDto {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DatasetRequestMessage {
    pub dataset: Urn,
}

impl CatalogMessageTrait for DatasetRequestMessage {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DatasetMessageDto {
    pub distribution: Urn,
    pub has_policy: Urn,
}

impl CatalogMessageTrait for DatasetMessageDto {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CatalogErrorDto {
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

impl CatalogMessageTrait for CatalogErrorDto {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CatalogMessageType {
    CatalogRequestMessage,
    Catalog,
    DatasetRequestMessage,
    Dataset,
    CatalogError,
}

impl Display for CatalogMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CatalogMessageType::CatalogRequestMessage => "CatalogRequestMessage".to_string(),
            CatalogMessageType::Catalog => "Catalog".to_string(),
            CatalogMessageType::DatasetRequestMessage => "DatasetRequestMessage".to_string(),
            CatalogMessageType::Dataset => "Dataset".to_string(),
            CatalogMessageType::CatalogError => "CatalogError".to_string(),
        };
        write!(f, "{}", str)
    }
}
