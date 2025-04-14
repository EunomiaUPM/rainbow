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

use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::repo::{
    EditCatalogModel, EditDataServiceModel, EditDatasetModel, EditDistributionModel,
    NewCatalogModel, NewDataServiceModel, NewDatasetModel, NewDistributionModel,
};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCatalogRequest {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Urn>,
    #[serde(rename = "foaf:homepage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foaf_home_page: Option<String>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
}

impl Into<NewCatalogModel> for NewCatalogRequest {
    fn into(self) -> NewCatalogModel {
        NewCatalogModel {
            id: self.id,
            foaf_home_page: self.foaf_home_page,
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
        }
    }
}

impl Into<EditCatalogModel> for NewCatalogRequest {
    fn into(self) -> EditCatalogModel {
        EditCatalogModel {
            foaf_home_page: self.foaf_home_page,
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDatasetRequest {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Urn>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
}

impl Into<NewDatasetModel> for NewDatasetRequest {
    fn into(self) -> NewDatasetModel {
        NewDatasetModel {
            id: self.id,
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dct_description: None,
        }
    }
}

impl Into<EditDatasetModel> for NewDatasetRequest {
    fn into(self) -> EditDatasetModel {
        EditDatasetModel {
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dct_description: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDataServiceRequest {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Urn>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dcat:endpointDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_endpoint_description: Option<String>,
    #[serde(rename = "dcat:endpointURL")]
    pub dcat_endpoint_url: String,
}

impl Into<NewDataServiceModel> for NewDataServiceRequest {
    fn into(self) -> NewDataServiceModel {
        NewDataServiceModel {
            id: self.id,
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dcat_endpoint_description: self.dcat_endpoint_description,
            dcat_endpoint_url: self.dcat_endpoint_url,
            dct_description: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditDataServiceRequest {
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dcat:endpointDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_endpoint_description: Option<String>,
    #[serde(rename = "dcat:endpointURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_endpoint_url: Option<String>,
}

impl Into<EditDataServiceModel> for EditDataServiceRequest {
    fn into(self) -> EditDataServiceModel {
        EditDataServiceModel {
            dct_conforms_to: self.dct_conforms_to,
            dct_creator: self.dct_creator,
            dct_title: self.dct_title,
            dcat_endpoint_description: self.dcat_endpoint_description,
            dcat_endpoint_url: self.dcat_endpoint_url,
            dct_description: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewDistributionRequest {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dcat:accessService")]
    pub dcat_access_service: Urn,
}

impl Into<NewDistributionModel> for NewDistributionRequest {
    fn into(self) -> NewDistributionModel {
        let id = self.id.as_ref().map(|x| get_urn_from_string(&x).unwrap());
        NewDistributionModel {
            id,
            dct_title: self.dct_title,
            dct_description: None,
            dcat_access_service: self.dcat_access_service.to_string(),
        }
    }
}

impl Into<EditDistributionModel> for NewDistributionRequest {
    fn into(self) -> EditDistributionModel {
        EditDistributionModel {
            dct_title: self.dct_title,
            dct_description: None,
            dcat_access_service: Option::from(self.dcat_access_service.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditDistributionRequest {
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dct_title: Option<String>,
    #[serde(rename = "dcat:accessService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcat_access_service: Option<Urn>,
}

impl Into<EditDistributionModel> for EditDistributionRequest {
    fn into(self) -> EditDistributionModel {
        EditDistributionModel {
            dct_title: self.dct_title,
            dct_description: None,
            dcat_access_service: self.dcat_access_service.map(|a| a.to_string()),
        }
    }
}
