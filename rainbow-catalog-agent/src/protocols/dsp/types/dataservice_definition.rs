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

use crate::protocols::dsp::types::EntityTypes;
use rainbow_common::dsp_common::context_field::ContextField;
use rainbow_common::dsp_common::odrl::OdrlOffer;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataService {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(flatten)]
    pub dcat: DataServiceDcatDeclaration,
    #[serde(flatten)]
    pub dct: DataServiceDctDeclaration,
    #[serde(rename = "hasPolicy")]
    pub odrl_offer: Vec<OdrlOffer>,
    #[serde(rename = "extraFields")]
    pub extra_fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataServiceDcatDeclaration {
    #[serde(rename = "theme")]
    pub theme: String,
    #[serde(rename = "keyword")]
    pub keyword: String,
    #[serde(rename = "endpointDescription")]
    pub endpoint_description: String,
    #[serde(rename = "endpointURL")]
    pub endpoint_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataServiceDctDeclaration {
    #[serde(rename = "conformsTo")]
    pub conforms_to: Option<String>,
    #[serde(rename = "creator")]
    pub creator: Option<String>,
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
}

impl Default for DataService {
    fn default() -> Self {
        DataService {
            context: ContextField::default(),
            _type: EntityTypes::DataService.to_string(),
            id: "".to_string(),
            dcat: DataServiceDcatDeclaration {
                theme: "".to_string(),
                keyword: "".to_string(),
                endpoint_description: "".to_string(),
                endpoint_url: "".to_string(),
            },
            dct: DataServiceDctDeclaration {
                conforms_to: None,
                creator: None,
                identifier: "".to_string(),
                issued: chrono::Utc::now().naive_utc(),
                modified: None,
                title: None,
                description: vec![],
            },
            odrl_offer: vec![],
            extra_fields: Value::default(),
        }
    }
}
