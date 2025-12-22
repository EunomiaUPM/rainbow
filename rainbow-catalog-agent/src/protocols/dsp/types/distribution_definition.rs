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

use crate::protocols::dsp::types::dataservice_definition::DataService;
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::dsp_common::context_field::ContextField;
use rainbow_common::dsp_common::odrl::OdrlOffer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Distribution {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(flatten)]
    pub dcat: DistributionDcatDeclaration,
    #[serde(flatten)]
    pub dct: DistributionDctDeclaration,
    #[serde(rename = "hasPolicy")]
    pub odrl_offer: Vec<OdrlOffer>,
    #[serde(rename = "extraFields")]
    pub extra_fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistributionDcatDeclaration {
    #[serde(rename = "accessService")]
    pub access_service: Option<DataService>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistributionDctDeclaration {
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "issued")]
    pub issued: chrono::NaiveDateTime,
    #[serde(rename = "modified")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "description")]
    pub description: Vec<String>,
    #[serde(rename = "formats")]
    pub formats: DctFormats,
}
