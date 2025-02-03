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
use super::rainbow_catalog_types::{
    EditDataServiceRequest, EditDistributionRequest, NewCatalogRequest, NewDataServiceRequest,
    NewDatasetRequest, NewDistributionRequest,
};
use crate::core::idsa_api::{
    dataservices_request_by_catalog, dataservices_request_by_id, dataset_request_by_catalog,
};
use crate::core::rainbow_catalog_err::CatalogError;
use crate::protocol::catalog_definition::Catalog;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::dataset_definition::Dataset;
use crate::protocol::distribution_definition::Distribution;
use crate::protocol::policies::EntityTypes;
use anyhow::bail;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::repo::CATALOG_REPO;
use serde_json::to_value;
use urn::Urn;

///
/// Catalog controllers
///
pub async fn catalog_request_by_id(id: Urn) -> anyhow::Result<Catalog> {
    let catalog = CATALOG_REPO.get_catalog_by_id(id.clone()).await.map_err(CatalogError::DbErr)?;

    match catalog {
        Some(catalog_entity) => {
            let mut catalog_out =
                Catalog::try_from(catalog_entity.clone()).map_err(CatalogError::ConversionError)?;
            let catalog_id = get_urn_from_string(&catalog_entity.id.clone())?;
            let odrl = CATALOG_REPO
                .get_all_odrl_offers_by_entity(catalog_id)
                .await
                .map_err(CatalogError::DbErr)?;
            catalog_out.odrl_offer = to_value(odrl)?;
            catalog_out.datasets = dataset_request_by_catalog(catalog_out.id.parse()?).await?;
            catalog_out.data_services =
                dataservices_request_by_catalog(catalog_out.id.parse()?).await?;
            Ok(catalog_out)
        }
        None => bail!(CatalogError::NotFound { id, entity: EntityTypes::Catalog.to_string() }),
    }
}

pub async fn post_catalog(input: NewCatalogRequest) -> anyhow::Result<Catalog> {
    let catalog_entity =
        CATALOG_REPO.create_catalog(input.into()).await.map_err(CatalogError::DbErr)?;
    let catalog = Catalog::try_from(catalog_entity).map_err(CatalogError::ConversionError)?;
    Ok(catalog)
}

pub async fn put_catalog(id: Urn, input: NewCatalogRequest) -> anyhow::Result<Catalog> {
    let catalog_entity = CATALOG_REPO.put_catalog_by_id(id.clone(), input.into()).await.map_err(
        |err| match err {
            rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                CatalogError::NotFound { id, entity: EntityTypes::Catalog.to_string() }
            }
            _ => CatalogError::DbErr(err),
        },
    )?;
    let catalog = Catalog::try_from(catalog_entity).map_err(CatalogError::ConversionError)?;
    Ok(catalog)
}

pub async fn delete_catalog(id: Urn) -> anyhow::Result<()> {
    let _ = CATALOG_REPO.delete_catalog_by_id(id.clone()).await.map_err(|err| match err {
        rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
            CatalogError::NotFound { id, entity: EntityTypes::Catalog.to_string() }
        }
        _ => CatalogError::DbErr(err),
    })?;
    Ok(())
}

///
/// Dataset controllers
///
pub async fn get_dataset_by_id(dataset_id: Urn) -> anyhow::Result<Dataset> {
    let dataset_entity =
        CATALOG_REPO.get_datasets_by_id(dataset_id.clone()).await.map_err(CatalogError::DbErr)?;
    match dataset_entity {
        Some(dataset_entity) => {
            let dataset =
                Dataset::try_from(dataset_entity).map_err(CatalogError::ConversionError)?;
            Ok(dataset)
        }
        None => bail!(CatalogError::NotFound {
            id: dataset_id,
            entity: EntityTypes::Dataset.to_string()
        }),
    }
}

pub async fn post_dataset(catalog_id: Urn, input: NewDatasetRequest) -> anyhow::Result<Dataset> {
    let dataset_entity =
        CATALOG_REPO.create_dataset(catalog_id, input.into()).await.map_err(CatalogError::DbErr)?;
    let dataset = Dataset::try_from(dataset_entity).map_err(CatalogError::ConversionError)?;
    Ok(dataset)
}

pub async fn put_dataset(
    catalog_id: Urn,
    dataset_id: Urn,
    input: NewDatasetRequest,
) -> anyhow::Result<Dataset> {
    let dataset_entity = CATALOG_REPO
        .put_datasets_by_id(dataset_id, input.into())
        .await
        .map_err(CatalogError::DbErr)?;
    let dataset = Dataset::try_from(dataset_entity).map_err(CatalogError::ConversionError)?;
    Ok(dataset)
}

pub async fn delete_dataset(catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<()> {
    let _ = CATALOG_REPO.delete_dataset_by_id(dataset_id).await.map_err(CatalogError::DbErr)?;
    Ok(())
}

///
/// Data service controllers
///
pub async fn get_dataservice_by_id(dataservice_id: Urn) -> anyhow::Result<Option<DataService>> {
    let dataservice_entity = CATALOG_REPO
        .get_data_service_by_id(dataservice_id.clone())
        .await
        .map_err(CatalogError::DbErr)?;
    match dataservice_entity {
        Some(dataservice_entity) => {
            let dataservice =
                DataService::try_from(dataservice_entity).map_err(CatalogError::ConversionError)?;
            Ok(Some(dataservice))
        }
        None => bail!(CatalogError::NotFound {
            id: dataservice_id,
            entity: EntityTypes::DataService.to_string()
        }),
    }
}

pub async fn post_dataservice(
    catalog_id: Urn,
    input: NewDataServiceRequest,
) -> anyhow::Result<DataService> {
    let dataservice_entity = CATALOG_REPO
        .create_data_service(catalog_id, input.into())
        .await
        .map_err(CatalogError::DbErr)?;
    let dataservice =
        DataService::try_from(dataservice_entity).map_err(CatalogError::ConversionError)?;
    Ok(dataservice)
}

pub async fn put_dataservice(
    catalog_id: Urn,
    dataservice_id: Urn,
    input: EditDataServiceRequest,
) -> anyhow::Result<DataService> {
    let dataservice_entity = CATALOG_REPO
        .put_data_service_by_id(dataservice_id, input.into())
        .await
        .map_err(CatalogError::DbErr)?;
    let dataservice =
        DataService::try_from(dataservice_entity).map_err(CatalogError::ConversionError)?;
    Ok(dataservice)
}

pub async fn delete_dataservice(catalog_id: Urn, dataset_id: Urn) -> anyhow::Result<()> {
    let _ =
        CATALOG_REPO.delete_data_service_by_id(dataset_id).await.map_err(CatalogError::DbErr)?;
    Ok(())
}

///
/// Distribution controllers
///
pub async fn get_distribution_by_id(distribution_id: Urn) -> anyhow::Result<Distribution> {
    let distribution = CATALOG_REPO
        .get_distribution_by_id(distribution_id.clone())
        .await
        .map_err(CatalogError::DbErr)?;
    match distribution {
        Some(distribution) => {
            let distribution =
                Distribution::try_from(distribution).map_err(CatalogError::ConversionError)?;
            Ok(distribution)
        }
        None => bail!(CatalogError::NotFound {
            id: distribution_id,
            entity: EntityTypes::Distribution.to_string()
        }),
    }
}

pub async fn get_distributions_by_dataset_id(dataset_id: Urn) -> anyhow::Result<Vec<Distribution>> {
    let distribution_entities = CATALOG_REPO
        .get_distributions_by_dataset_id(dataset_id)
        .await
        .map_err(CatalogError::DbErr)?;
    let distributions = distribution_entities
        .iter()
        .map(|distribution_entity| {
            let mut distribution = Distribution::try_from(distribution_entity.clone())
                .map_err(CatalogError::ConversionError)?;
            Ok(distribution)
        })
        .collect::<anyhow::Result<Vec<Distribution>>>()?;
    Ok(distributions)
}

pub async fn post_distribution(
    catalog_id: Urn,
    dataset_id: Urn,
    input: NewDistributionRequest,
) -> anyhow::Result<Distribution> {
    let distribution_entity = CATALOG_REPO
        .create_distribution(dataset_id, input.clone().into())
        .await
        .map_err(CatalogError::DbErr)?;
    let mut distribution =
        Distribution::try_from(distribution_entity).map_err(CatalogError::ConversionError)?;
    distribution.dcat.access_service =
        dataservices_request_by_id(input.dcat_access_service.to_string()).await?;
    Ok(distribution)
}

pub async fn put_distribution(
    catalog_id: Urn,
    dataservice_id: Urn,
    distribution_id: Urn,
    input: EditDistributionRequest,
) -> anyhow::Result<Distribution> {
    let distribution_entity = CATALOG_REPO
        .put_distribution_by_id(distribution_id, input.into())
        .await
        .map_err(CatalogError::DbErr)?;
    let mut distribution = Distribution::try_from(distribution_entity.clone())
        .map_err(CatalogError::ConversionError)?;
    distribution.dcat.access_service =
        dataservices_request_by_id(distribution_entity.dcat_access_service.unwrap()).await?;
    Ok(distribution)
}

pub async fn delete_distribution(
    catalog_id: Urn,
    dataservice_id: Urn,
    distribution_id: Urn,
) -> anyhow::Result<()> {
    let _ = CATALOG_REPO
        .delete_distribution_by_id(distribution_id)
        .await
        .map_err(CatalogError::DbErr)?;
    Ok(())
}
