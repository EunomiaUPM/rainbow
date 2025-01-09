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

use crate::core::ll_api::{
    dataservices_request_by_catalog, dataservices_request_by_id, dataset_request_by_catalog,
};
use crate::protocol::catalog_definition::Catalog;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::dataset_definition::Dataset;
use crate::protocol::distribution_definition::Distribution;
use anyhow::bail;
use axum::routing::get;
use clap::builder::Str;
use rainbow_common::opt_urn_serde;
use rainbow_common::urn_serde;
use rainbow_common::utils;
use rainbow_common::utils::get_urn;
use rainbow_common::config::database::get_db_connection;
use rainbow_db::catalog::entities::{catalog, dataset, distribution};
use rainbow_db::catalog::entities::{dataservice, odrl_offer};
use sea_orm::{ActiveValue, ColumnTrait};
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use axum::http::Uri;
use tracing::info;
use urn::Urn;

pub async fn catalog_request_by_id(id: String) -> anyhow::Result<Catalog> {
    let db_connection = get_db_connection().await;
    let catalog =
        catalog::Entity::find().filter(catalog::Column::Id.eq(id.to_string())).one(db_connection).await?;

    match catalog {
        Some(catalog_entity) => {
            let mut catalog = Catalog::try_from(catalog_entity.clone()).unwrap();
            let catalog_odrl_from_db = odrl_offer::Entity::find()
                .filter(odrl_offer::Column::Entity.eq(catalog_entity.id))
                .all(db_connection)
                .await?;
            catalog.odrl_offer = to_value(catalog_odrl_from_db)?;
            catalog.datasets = dataset_request_by_catalog(catalog.id.parse()?).await?;
            catalog.data_services = dataservices_request_by_catalog(catalog.id.parse()?).await?;
            Ok(catalog)
        }
        None => Err(anyhow::anyhow!("Catalog does not exist")),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCatalogRequest {
    #[serde(rename = "@id")]
    #[serde(with="opt_urn_serde")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Urn>,
    #[serde(rename = "foaf:homepage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    foaf_home_page: Option<String>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_title: Option<String>,
}



pub async fn post_catalog(input: NewCatalogRequest) -> anyhow::Result<Catalog> {
    let db_connection = get_db_connection().await;
    let input_string = input.id.clone().unwrap();
    
    let urn = get_urn(input.id);

    let new_catalog = catalog::ActiveModel {
        id: ActiveValue::Set(urn.to_string()),
        foaf_home_page: ActiveValue::Set(input.foaf_home_page),
        dct_conforms_to: ActiveValue::Set(input.dct_conforms_to),
        dct_creator: ActiveValue::Set(input.dct_creator),
        dct_identifier: ActiveValue::Set(Some(urn.to_string())),
        dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        dct_modified: Default::default(),
        dct_title: ActiveValue::Set(input.dct_title),
        dspace_participant_id: Default::default(), // TODO get participant id
    };

    let catalog_entity =
        catalog::Entity::insert(new_catalog).exec_with_returning(db_connection).await?;

    let catalog = Catalog::try_from(catalog_entity).unwrap();
    Ok(catalog)
}

pub async fn put_catalog(id: String, input: NewCatalogRequest) -> anyhow::Result<Catalog> {
    let db_connection = get_db_connection().await;
    let catalog: Option<catalog::Model> =
        catalog::Entity::find_by_id(id.to_string()).one(db_connection).await?;

    if catalog.is_none() {
        bail!("Catalog does not exist"); // TODO 404
    }

    let mut new_catalog: catalog::ActiveModel = catalog.unwrap().into();

    if let Some(foaf_home_page) = input.foaf_home_page {
        new_catalog.foaf_home_page = ActiveValue::Set(Some(foaf_home_page));
    }
    if let Some(dct_conforms_to) = input.dct_conforms_to {
        new_catalog.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
    }
    if let Some(dct_creator) = input.dct_creator {
        new_catalog.dct_creator = ActiveValue::Set(Some(dct_creator));
    }
    if let Some(dct_title) = input.dct_title {
        new_catalog.dct_title = ActiveValue::Set(Some(dct_title));
    }
    new_catalog.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

    let catalog_entity = catalog::Entity::update(new_catalog).exec(db_connection).await?;

    let catalog = Catalog::try_from(catalog_entity).unwrap();
    Ok(catalog)
}

pub async fn delete_catalog(id: String) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    let catalog = catalog::Entity::delete_by_id(id.to_string()).exec(db_connection).await?;
    if catalog.rows_affected == 0 {
        bail!("Catalog does not exist");
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDatasetRequest {
    #[serde(rename = "@id")]
    #[serde(with="opt_urn_serde")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Urn>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_title: Option<String>,
}
pub async fn post_dataset(catalog_id: String, input: NewDatasetRequest) -> anyhow::Result<Dataset> {
    let db_connection = get_db_connection().await;
    let urn = get_urn(input.id);
    let new_dataset = dataset::ActiveModel {
        id: ActiveValue::Set(urn.to_string()),
        dct_conforms_to: ActiveValue::Set(input.dct_conforms_to),
        dct_creator: ActiveValue::Set(input.dct_creator),
        dct_identifier: ActiveValue::Set(Some(urn.to_string())),
        dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        dct_modified: Default::default(),
        dct_title: ActiveValue::Set(input.dct_title),
        dct_description: Default::default(),
        catalog_id: ActiveValue::Set(catalog_id),
    };

    let dataset_entity =
        dataset::Entity::insert(new_dataset).exec_with_returning(db_connection).await?;

    let dataset = Dataset::try_from(dataset_entity).unwrap();
    Ok(dataset)
}

pub async fn put_dataset(
    catalog_id: String,
    dataset_id: String,
    input: NewDatasetRequest,
) -> anyhow::Result<Dataset> {
    let db_connection = get_db_connection().await;
    let dataset: Option<dataset::Model> =
        dataset::Entity::find_by_id(dataset_id).one(db_connection).await?;
    // TODO check if dataset is in catalog
    if dataset.is_none() {
        bail!("Dataset does not exist"); // TODO 404
    }

    let mut new_dataset: dataset::ActiveModel = dataset.unwrap().into();

    if let Some(dct_conforms_to) = input.dct_conforms_to {
        new_dataset.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
    }
    if let Some(dct_creator) = input.dct_creator {
        new_dataset.dct_creator = ActiveValue::Set(Some(dct_creator));
    }
    if let Some(dct_title) = input.dct_title {
        new_dataset.dct_title = ActiveValue::Set(Some(dct_title));
    }
    new_dataset.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

    let dataset_entity = dataset::Entity::update(new_dataset).exec(db_connection).await?;

    let dataset = Dataset::try_from(dataset_entity).unwrap();
    Ok(dataset)
}

pub async fn delete_dataset(catalog_id: String, dataset_id: String) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    let dataset = dataset::Entity::delete_by_id(dataset_id.to_string()).exec(db_connection).await?;
    if dataset.rows_affected == 0 {
        bail!("Dataset does not exist");
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDataServiceRequest {
    #[serde(rename = "@id")]
    #[serde(with="opt_urn_serde")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Urn>,
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_title: Option<String>,
    #[serde(rename = "dcat:endpointDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dcat_endpoint_description: Option<String>,
    #[serde(rename = "dcat:endpointURL")]
    dcat_endpoint_url: String,
}

pub async fn post_dataservice(
    catalog_id: String,
    input: NewDataServiceRequest,
) -> anyhow::Result<DataService> {
    let db_connection = get_db_connection().await;
    let urn = get_urn(input.id);
    let new_dataservice = dataservice::ActiveModel {
        id: ActiveValue::Set(urn.to_string()),
        dcat_endpoint_description: ActiveValue::Set(input.dcat_endpoint_description),
        dcat_endpoint_url: ActiveValue::Set(input.dcat_endpoint_url),
        dct_conforms_to: ActiveValue::Set(input.dct_conforms_to),
        dct_creator: ActiveValue::Set(input.dct_creator),
        dct_identifier: ActiveValue::Set(Some(urn.to_string())),
        dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        dct_modified: Default::default(),
        dct_title: ActiveValue::Set(input.dct_title),
        dct_description: Default::default(),
        catalog_id: ActiveValue::Set(catalog_id),
    };

    let dataservice_entity =
        dataservice::Entity::insert(new_dataservice).exec_with_returning(db_connection).await?;

    let dataservice = DataService::try_from(dataservice_entity).unwrap();
    Ok(dataservice)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditDataServiceRequest {
    #[serde(rename = "dct:conformsTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_conforms_to: Option<String>,
    #[serde(rename = "dct:creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_creator: Option<String>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_title: Option<String>,
    #[serde(rename = "dcat:endpointDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dcat_endpoint_description: Option<String>,
    #[serde(rename = "dcat:endpointURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dcat_endpoint_url: Option<String>,
}

pub async fn put_dataservice(
    catalog_id: String,
    dataservice_id: String,
    input: EditDataServiceRequest,
) -> anyhow::Result<DataService> {
    let db_connection = get_db_connection().await;
    let dataservice: Option<dataservice::Model> =
        dataservice::Entity::find_by_id(dataservice_id).one(db_connection).await?;
    // TODO check if dataset is in catalog
    if dataservice.is_none() {
        bail!("Data service does not exist"); // TODO 404
    }

    let mut new_dataservice: dataservice::ActiveModel = dataservice.unwrap().into();

    if let Some(dct_conforms_to) = input.dct_conforms_to {
        new_dataservice.dct_conforms_to = ActiveValue::Set(Some(dct_conforms_to));
    }
    if let Some(dct_creator) = input.dct_creator {
        new_dataservice.dct_creator = ActiveValue::Set(Some(dct_creator));
    }
    if let Some(dct_title) = input.dct_title {
        new_dataservice.dct_title = ActiveValue::Set(Some(dct_title));
    }
    if let Some(dcat_endpoint_description) = input.dcat_endpoint_description {
        new_dataservice.dcat_endpoint_description =
            ActiveValue::Set(Some(dcat_endpoint_description));
    }
    if let Some(dcat_endpoint_url) = input.dcat_endpoint_url {
        new_dataservice.dcat_endpoint_url = ActiveValue::Set(dcat_endpoint_url);
    }
    new_dataservice.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

    let dataservice_entity =
        dataservice::Entity::update(new_dataservice).exec(db_connection).await?;

    let dataservice = DataService::try_from(dataservice_entity).unwrap();
    Ok(dataservice)
}

pub async fn delete_dataservice(catalog_id: String, dataset_id: String) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    let dataservice = dataservice::Entity::delete_by_id(dataset_id).exec(db_connection).await?;
    if dataservice.rows_affected == 0 {
        bail!("Data service does not exist");
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDistributionRequest {
    #[serde(rename = "@id")]
    #[serde(with="opt_urn_serde")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Urn>,
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_title: Option<String>,
    #[serde(with="urn_serde")]
    #[serde(rename = "dcat:accessService")]
    dcat_access_service: Urn,
}

pub async fn post_distribution(
    catalog_id: String,
    dataset_id: String,
    input: NewDistributionRequest,
) -> anyhow::Result<Distribution> {
    let db_connection = get_db_connection().await;
    let urn = get_urn(input.id);
    let new_distribution = distribution::ActiveModel {
        id: ActiveValue::Set(urn.to_string()),
        dct_issued: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        dct_modified: Default::default(),
        dct_title: ActiveValue::Set(input.dct_title),
        dct_description: Default::default(),
        dcat_access_service: ActiveValue::Set(input.dcat_access_service.to_string()),
        dataset_id: ActiveValue::Set(dataset_id),
    };

    let distribution_entity =
        distribution::Entity::insert(new_distribution).exec_with_returning(db_connection).await?;

    let mut distribution = Distribution::try_from(distribution_entity).unwrap();
    distribution.dcat.access_service =
        dataservices_request_by_id(input.dcat_access_service.to_string()).await?;
    Ok(distribution)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditDistributionRequest {
    #[serde(rename = "dct:title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dct_title: Option<String>,
    #[serde(rename = "dcat:accessService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dcat_access_service: Option<String>,
}

pub async fn put_distribution(
    catalog_id: String,
    dataservice_id: String,
    distribution_id: String,
    input: EditDistributionRequest,
) -> anyhow::Result<Distribution> {
    let db_connection = get_db_connection().await;
    let distribution: Option<distribution::Model> =
        distribution::Entity::find_by_id(distribution_id).one(db_connection).await?;
    // TODO check if dataset is in catalog

    if distribution.is_none() {
        bail!("Distribution does not exist"); // TODO 404
    }

    let mut new_distribution: distribution::ActiveModel = distribution.unwrap().into();

    if let Some(dct_title) = input.dct_title {
        new_distribution.dct_title = ActiveValue::Set(Some(dct_title));
    }
    if let Some(dcat_access_service) = input.dcat_access_service {
        new_distribution.dcat_access_service = ActiveValue::Set(dcat_access_service);
    }

    new_distribution.dct_modified = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

    let distribution_entity =
        distribution::Entity::update(new_distribution).exec(db_connection).await?;

    // here
    let mut distribution = Distribution::try_from(distribution_entity.clone()).unwrap();
    distribution.dcat.access_service =
        dataservices_request_by_id(distribution_entity.dcat_access_service).await?;
    Ok(distribution)
}

pub async fn delete_distribution(
    catalog_id: String,
    dataservice_id: String,
    distribution_id: String,
) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    let distribution =
        distribution::Entity::delete_by_id(distribution_id.to_string()).exec(db_connection).await?;
    if distribution.rows_affected == 0 {
        bail!("Distribution does not exist");
    }
    Ok(())
}
