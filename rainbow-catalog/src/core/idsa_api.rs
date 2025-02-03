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

use anyhow::bail;
use rainbow_common::config::database::get_db_connection;
use rainbow_db::catalog::entities::{catalog, dataset, distribution};
use rainbow_db::catalog::entities::{dataservice, odrl_offer};

use crate::core::rainbow_catalog_err::CatalogError;
use crate::protocol::catalog_definition::Catalog;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::dataset_definition::Dataset;
use crate::protocol::distribution_definition::Distribution;
use rainbow_common::utils::get_urn_from_string;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogRequestMessage {
    #[serde(rename = "@context")] // TODO Define validators
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:filter")]
    pub filter: Option<Value>, // TODO Define further
}

pub async fn dataset_request(dataset_id: Urn) -> anyhow::Result<Dataset> {
    let db_connection = get_db_connection().await;
    let datasets_out: Vec<Dataset> = vec![];
    let datasets_from_db = dataset::Entity::find()
        .filter(dataset::Column::Id.eq(dataset_id.to_string()))
        .one(db_connection)
        .await?;

    match datasets_from_db {
        Some(dataset_from_db) => {
            let mut dataset = Dataset::try_from(dataset_from_db.clone()).unwrap();
            // odrl
            let dataset_odrl_from_db = odrl_offer::Entity::find()
                .filter(odrl_offer::Column::Entity.eq(dataset_from_db.id))
                .all(db_connection)
                .await?;
            dataset.odrl_offer = to_value(dataset_odrl_from_db)?;
            dataset.distribution = distributions_request_by_dataset(
                dataset.id.parse()?,
                dataset_from_db.catalog_id.parse()?,
            )
                .await?;
            Ok(dataset)
        }
        None => bail!("dataset not found"),
    }
}

pub async fn dataset_request_by_catalog(catalog_id: Urn) -> anyhow::Result<Vec<Dataset>> {
    let db_connection = get_db_connection().await;
    let mut datasets_out: Vec<Dataset> = vec![];
    let datasets_from_db = dataset::Entity::find()
        .filter(dataset::Column::CatalogId.eq(catalog_id.clone().to_string()))
        .all(db_connection)
        .await?;

    for dataset_entity in datasets_from_db {
        let mut dataset = Dataset::try_from(dataset_entity.clone()).unwrap();

        // odrl
        let dataset_odrl_from_db = odrl_offer::Entity::find()
            .filter(odrl_offer::Column::Entity.eq(dataset_entity.id))
            .all(db_connection)
            .await?;
        dataset.odrl_offer = to_value(dataset_odrl_from_db)?;
        dataset.distribution =
            distributions_request_by_dataset(dataset.id.parse()?, catalog_id.clone()).await?;
        datasets_out.push(dataset);
    }

    Ok(datasets_out)
}

pub async fn dataservices_request_by_catalog(catalog_id: Urn) -> anyhow::Result<Vec<DataService>> {
    let db_connection = get_db_connection().await;
    let mut dataservices_out: Vec<DataService> = vec![];
    let dataservices_from_db = dataservice::Entity::find()
        .filter(dataservice::Column::CatalogId.eq(catalog_id.to_string()))
        .all(db_connection)
        .await?;

    for dataservice_entity in dataservices_from_db {
        let mut dataservices = DataService::try_from(dataservice_entity.clone()).unwrap();

        // odrl
        let dataset_odrl_from_db = odrl_offer::Entity::find()
            .filter(odrl_offer::Column::Entity.eq(dataservice_entity.id))
            .all(db_connection)
            .await?;
        dataservices.odrl_offer = to_value(dataset_odrl_from_db)?;
        dataservices_out.push(dataservices);
    }

    Ok(dataservices_out)
}

pub async fn dataservices_request_by_id(
    dataservice_id: Urn,
) -> anyhow::Result<Option<DataService>> {
    let db_connection = get_db_connection().await;
    let dataservice_from_db = dataservice::Entity::find_by_id(dataservice_id.clone().to_string())
        .one(db_connection)
        .await?;

    let dataservice = match dataservice_from_db {
        Some(d) => Some(DataService::try_from(d)?),
        None => bail!(CatalogError::DataServiceNotFoundForDistribution { id: dataservice_id }),
    };
    Ok(dataservice)
}

pub async fn distributions_request_by_dataset(
    dataset_id: Urn,
    catalog_id: Urn,
) -> anyhow::Result<Vec<Distribution>> {
    let db_connection = get_db_connection().await;
    let mut distributions_out: Vec<Distribution> = vec![];
    let distributions_from_db = distribution::Entity::find()
        .filter(distribution::Column::DatasetId.eq(dataset_id.to_string()))
        .all(db_connection)
        .await?;

    for distribution_entity in distributions_from_db {
        let mut distribution = Distribution::try_from(distribution_entity.clone())?;

        // odrl
        let distribution_odrl_from_db = odrl_offer::Entity::find()
            .filter(odrl_offer::Column::Entity.eq(distribution_entity.id))
            .all(db_connection)
            .await?;
        distribution.odrl_offer = to_value(distribution_odrl_from_db)?;
        // dataservice
        let dcat_access_service = get_urn_from_string(&distribution_entity.dcat_access_service)?;
        distribution.dcat.access_service = dataservices_request_by_id(dcat_access_service).await?;
        distributions_out.push(distribution);
    }

    Ok(distributions_out)
}

pub async fn catalog_request() -> anyhow::Result<Vec<Catalog>> {
    let db_connection = get_db_connection().await;
    let mut catalogs_out: Vec<Catalog> = vec![];
    let catalogs_from_db = catalog::Entity::find().limit(5).all(db_connection).await?;

    for catalog_entity in catalogs_from_db {
        let mut catalog = Catalog::try_from(catalog_entity.clone()).unwrap();

        // odrl
        let catalog_odrl_from_db = odrl_offer::Entity::find()
            .filter(odrl_offer::Column::Entity.eq(catalog_entity.id))
            .all(db_connection)
            .await?;
        catalog.odrl_offer = to_value(catalog_odrl_from_db)?;
        catalog.datasets = dataset_request_by_catalog(catalog.id.parse()?).await?;
        catalog.data_services = dataservices_request_by_catalog(catalog.id.parse()?).await?;

        catalogs_out.push(catalog);
    }

    Ok(catalogs_out)
}

pub async fn catalog_request_by_id() -> anyhow::Result<Vec<Catalog>> {
    let db_connection = get_db_connection().await;
    let mut catalogs_out: Vec<Catalog> = vec![];
    let catalogs_from_db = catalog::Entity::find().limit(5).all(db_connection).await?;

    for catalog_entity in catalogs_from_db {
        let mut catalog = Catalog::try_from(catalog_entity.clone()).unwrap();

        // odrl
        let catalog_odrl_from_db = odrl_offer::Entity::find()
            .filter(odrl_offer::Column::Entity.eq(catalog_entity.id))
            .all(db_connection)
            .await?;
        catalog.odrl_offer = to_value(catalog_odrl_from_db)?;
        catalog.datasets = dataset_request_by_catalog(catalog.id.parse()?).await?;
        catalog.data_services = dataservices_request_by_catalog(catalog.id.parse()?).await?;

        catalogs_out.push(catalog);
    }

    Ok(catalogs_out)
}
