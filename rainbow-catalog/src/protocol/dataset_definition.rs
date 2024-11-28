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

use crate::protocol::distribution_definition::Distribution;
use rainbow_db::catalog::entities::dataset;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dataset {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(flatten)]
    pub dcat: DatasetDcatDeclaration,
    #[serde(flatten)]
    pub dct: DatasetDctDeclaration,
    #[serde(rename = "odrl:hasPolicy")]
    pub odrl_offer: serde_json::Value,
    #[serde(rename = "odrl:extraFields")]
    pub extra_fields: serde_json::Value,
    #[serde(rename = "dcat:distribution")]
    pub distribution: Vec<Distribution>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatasetDcatDeclaration {
    #[serde(rename = "dcat:theme")]
    pub theme: String,
    #[serde(rename = "dcat:keyword")]
    pub keyword: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatasetDctDeclaration {
    #[serde(rename = "dct:conformsTo")]
    pub conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    pub creator: Option<String>,
    #[serde(rename = "dct:identifier")]
    pub identifier: String,
    #[serde(rename = "dct:issued")]
    pub issued: chrono::NaiveDateTime,
    #[serde(rename = "dct:modified")]
    pub modified: Option<chrono::NaiveDateTime>,
    #[serde(rename = "dct:title")]
    pub title: Option<String>,
    #[serde(rename = "dct:description")]
    pub description: Vec<String>,
}

impl TryFrom<dataset::Model> for Dataset {
    type Error = anyhow::Error;

    fn try_from(dataset_model: dataset::Model) -> Result<Self, Self::Error> {
        Ok(Dataset {
            context: "https://w3id.org/dspace/2024/1/context.json".to_string(),
            _type: "dcat:Dataset".to_string(),
            id: dataset_model.id.to_string(),
            dcat: DatasetDcatDeclaration { theme: "".to_string(), keyword: "".to_string() },
            dct: DatasetDctDeclaration {
                conforms_to: dataset_model.dct_conforms_to,
                creator: dataset_model.dct_creator,
                identifier: dataset_model
                    .dct_identifier
                    .unwrap_or_else(|| dataset_model.id.to_string()),
                issued: dataset_model.dct_issued,
                modified: dataset_model.dct_modified,
                title: dataset_model.dct_title,
                description: vec![],
            },
            odrl_offer: Value::default(),
            extra_fields: Value::default(),
            distribution: vec![],
        })
    }
}
