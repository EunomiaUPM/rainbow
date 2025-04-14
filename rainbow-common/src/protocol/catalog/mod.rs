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
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod catalog_definition;
pub mod dataservice_definition;
pub mod dataset_definition;
pub mod distribution_definition;
pub mod catalog_protocol_types;

#[derive(Debug, Serialize, Deserialize)]
pub enum CatalogProtocolEntities {
    #[serde(rename = "Catalog")]
    Catalog,
    #[serde(rename = "DataService")]
    DataService,
    #[serde(rename = "Dataset")]
    Dataset,
    #[serde(rename = "Distribution")]
    Distribution,
}

impl Display for CatalogProtocolEntities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CatalogProtocolEntities::Catalog => "Catalog".to_string(),
            CatalogProtocolEntities::Dataset => "Dataset".to_string(),
            CatalogProtocolEntities::DataService => "DataService".to_string(),
            CatalogProtocolEntities::Distribution => "Distribution".to_string(),
        };
        write!(f, "{}", str)
    }
}

