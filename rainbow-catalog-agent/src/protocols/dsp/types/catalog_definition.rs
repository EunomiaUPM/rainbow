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

// use rainbow_db::catalog::entities::catalog;
use crate::protocols::dsp::types::dataservice_definition::DataService;
use crate::protocols::dsp::types::dataset_definition::Dataset;
use rainbow_common::dsp_common::context_field::ContextField;
use rainbow_common::dsp_common::odrl::OdrlOffer;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "@id")]
    pub id: Urn,
    #[serde(flatten)]
    pub foaf: CatalogFoafDeclaration,
    #[serde(flatten)]
    pub dcat: CatalogDcatDeclaration,
    #[serde(flatten)]
    pub dct: CatalogDctDeclaration,
    #[serde(flatten)]
    pub dspace: CatalogDSpaceDeclaration,
    #[serde(rename = "hasPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odrl_offer: Option<Vec<OdrlOffer>>,
    #[serde(rename = "extraFields")]
    pub extra_fields: serde_json::Value,
    #[serde(rename = "catalog")]
    pub catalogs: Vec<Catalog>,
    #[serde(rename = "dataset")]
    pub datasets: Vec<Dataset>,
    #[serde(rename = "service")]
    pub data_services: Vec<DataService>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogFoafDeclaration {
    #[serde(rename = "homepage")]
    pub homepage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogDcatDeclaration {
    #[serde(rename = "theme")]
    pub theme: String,
    #[serde(rename = "keyword")]
    pub keyword: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogDctDeclaration {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogDSpaceDeclaration {
    #[serde(rename = "participantId")]
    pub participant_id: Option<String>,
}
