use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_entities::rainbow_catalog_types::{EditDistributionRequest, NewDistributionRequest};
use crate::core::rainbow_entities::RainbowDistributionTrait;
use crate::protocol::dataservice_definition::DataService;
use crate::protocol::distribution_definition::Distribution;
use crate::protocol::policies::EntityTypes;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogDistributionService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
}

impl<T> RainbowCatalogDistributionService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
    async fn data_services_request_by_id(&self, data_service_id: Urn) -> anyhow::Result<Option<DataService>> {
        let data_service = self.repo.get_data_service_by_id(data_service_id).await.map_err(CatalogError::DbErr)?;
        let data_service = data_service.map(|m| DataService::try_from(m).unwrap());
        Ok(data_service)
    }
}

#[async_trait]
impl<T> RainbowDistributionTrait for RainbowCatalogDistributionService<T>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + Send + Sync + 'static,
{
    async fn get_distribution_by_id(&self, distribution_id: Urn) -> anyhow::Result<Distribution> {
        let distribution =
            self.repo.get_distribution_by_id(distribution_id.clone()).await.map_err(CatalogError::DbErr)?;
        match distribution {
            Some(distribution) => {
                let distribution = Distribution::try_from(distribution).map_err(CatalogError::ConversionError)?;
                Ok(distribution)
            }
            None => {
                bail!(CatalogError::NotFound { id: distribution_id, entity: EntityTypes::Distribution.to_string() })
            }
        }
    }

    async fn get_distributions_by_dataset_id(&self, dataset_id: Urn) -> anyhow::Result<Vec<Distribution>> {
        let distribution_entities =
            self.repo.get_distributions_by_dataset_id(dataset_id).await.map_err(CatalogError::DbErr)?;
        let distributions = distribution_entities
            .iter()
            .map(|distribution_entity| {
                let mut distribution =
                    Distribution::try_from(distribution_entity.clone()).map_err(CatalogError::ConversionError)?;
                Ok(distribution)
            })
            .collect::<anyhow::Result<Vec<Distribution>>>()?;
        Ok(distributions)
    }

    async fn post_distribution(
        &self,
        catalog_id: Urn,
        dataset_id: Urn,
        input: NewDistributionRequest,
    ) -> anyhow::Result<Distribution> {
        let distribution_entity =
            self.repo.create_distribution(catalog_id.clone(), dataset_id.clone(), input.clone().into()).await.map_err(
                |err| match err {
                    rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                        CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                    }
                    rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                        CatalogError::NotFound { id: dataset_id, entity: EntityTypes::Dataset.to_string() }
                    }
                    _ => CatalogError::DbErr(err),
                },
            )?;
        let mut distribution = Distribution::try_from(distribution_entity).map_err(CatalogError::ConversionError)?;
        distribution.dcat.access_service = self.data_services_request_by_id(input.dcat_access_service).await?;
        Ok(distribution)
    }

    async fn put_distribution(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
        distribution_id: Urn,
        input: EditDistributionRequest,
    ) -> anyhow::Result<Distribution> {
        let distribution_entity = self
            .repo
            .put_distribution_by_id(
                catalog_id.clone(),
                data_service_id.clone(),
                distribution_id.clone(),
                input.clone().into(),
            )
            .await
            .map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                    CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                    CatalogError::NotFound { id: data_service_id, entity: EntityTypes::Dataset.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DistributionNotFound => {
                    CatalogError::NotFound { id: distribution_id, entity: EntityTypes::Distribution.to_string() }
                }
                _ => CatalogError::DbErr(err),
            })?;
        let mut distribution =
            Distribution::try_from(distribution_entity.clone()).map_err(CatalogError::ConversionError)?;

        if let Some(dcat_access_service) = input.dcat_access_service {
            let dcat_access_service = get_urn_from_string(&dcat_access_service)?;
            distribution.dcat.access_service = self.data_services_request_by_id(dcat_access_service).await?;
        }

        Ok(distribution)
    }

    async fn delete_distribution(
        &self,
        catalog_id: Urn,
        data_service_id: Urn,
        distribution_id: Urn,
    ) -> anyhow::Result<()> {
        let _ = self
            .repo
            .delete_distribution_by_id(
                catalog_id.clone(),
                data_service_id.clone(),
                distribution_id.clone(),
            )
            .await
            .map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogNotFound => {
                    CatalogError::NotFound { id: catalog_id, entity: EntityTypes::Catalog.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DatasetNotFound => {
                    CatalogError::NotFound { id: data_service_id, entity: EntityTypes::Dataset.to_string() }
                }
                rainbow_db::catalog::repo::CatalogRepoErrors::DistributionNotFound => {
                    CatalogError::NotFound { id: distribution_id, entity: EntityTypes::Distribution.to_string() }
                }
                _ => CatalogError::DbErr(err),
            })?;
        Ok(())
    }
}
