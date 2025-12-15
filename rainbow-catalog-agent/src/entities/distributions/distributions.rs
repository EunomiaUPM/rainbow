use crate::data::factory_trait::CatalogAgentRepoTrait;
use crate::entities::distributions::{
    DistributionDto, DistributionEntityTrait, EditDistributionDto, NewDistributionDto,
};
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct DistributionEntities {
    repo: Arc<dyn CatalogAgentRepoTrait>,
}

impl DistributionEntities {
    pub fn new(repo: Arc<dyn CatalogAgentRepoTrait>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl DistributionEntityTrait for DistributionEntities {
    async fn get_all_distributions(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<DistributionDto>> {
        let distributions =
            self.repo.get_distribution_repo().get_all_distributions(limit, page).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let mut dtos = Vec::with_capacity(distributions.len());
        for c in distributions {
            let dto: DistributionDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_batch_distributions(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<DistributionDto>> {
        let distributions = self.repo.get_distribution_repo().get_batch_distributions(ids).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let mut dtos = Vec::with_capacity(distributions.len());
        for c in distributions {
            let dto: DistributionDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_distributions_by_dataset_id(&self, dataset_id: &Urn) -> anyhow::Result<Vec<DistributionDto>> {
        let distributions =
            self.repo.get_distribution_repo().get_distributions_by_dataset_id(dataset_id).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let mut dtos = Vec::with_capacity(distributions.len());
        for c in distributions {
            let dto: DistributionDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_distribution_by_dataset_id_and_dct_format(
        &self,
        dataset_id: &Urn,
        dct_formats: &DctFormats,
    ) -> anyhow::Result<DistributionDto> {
        let distribution = self
            .repo
            .get_distribution_repo()
            .get_distribution_by_dataset_id_and_dct_format(dataset_id, dct_formats)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let dto = distribution.into();
        Ok(dto)
    }

    async fn get_distribution_by_id(&self, distribution_id: &Urn) -> anyhow::Result<Option<DistributionDto>> {
        let distribution =
            self.repo.get_distribution_repo().get_distribution_by_id(distribution_id).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let dto = distribution.map(|d| d.into());
        Ok(dto)
    }

    async fn put_distribution_by_id(
        &self,
        distribution_id: &Urn,
        edit_distribution_model: &EditDistributionDto,
    ) -> anyhow::Result<DistributionDto> {
        let edit_model = edit_distribution_model.clone().into();
        let distribution =
            self.repo.get_distribution_repo().put_distribution_by_id(distribution_id, &edit_model).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;
        let dto = distribution.into();
        Ok(dto)
    }

    async fn create_distribution(
        &self,
        new_distribution_model: &NewDistributionDto,
    ) -> anyhow::Result<DistributionDto> {
        let new_model = new_distribution_model.clone().into();
        let distribution = self.repo.get_distribution_repo().create_distribution(&new_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = distribution.into();
        Ok(dto)
    }

    async fn delete_distribution_by_id(&self, distribution_id: &Urn) -> anyhow::Result<()> {
        self.repo.get_distribution_repo().delete_distribution_by_id(distribution_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        Ok(())
    }
}
