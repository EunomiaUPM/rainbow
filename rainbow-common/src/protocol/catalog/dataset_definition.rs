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

use crate::protocol::catalog::distribution_definition::Distribution;
use crate::protocol::context_field::ContextField;
use crate::protocol::contract::contract_odrl::OdrlOffer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dataset {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(flatten)]
    pub dcat: DatasetDcatDeclaration,
    #[serde(flatten)]
    pub dct: DatasetDctDeclaration,
    #[serde(rename = "hasPolicy")]
    pub odrl_offer: Vec<OdrlOffer>,
    #[serde(rename = "extraFields")]
    pub extra_fields: serde_json::Value,
    #[serde(rename = "distribution")]
    pub distribution: Vec<Distribution>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatasetDcatDeclaration {
    #[serde(rename = "theme")]
    pub theme: String,
    #[serde(rename = "keyword")]
    pub keyword: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatasetDctDeclaration {
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


