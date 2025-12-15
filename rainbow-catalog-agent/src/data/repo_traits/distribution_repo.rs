use crate::data::entities::distribution;
use crate::data::entities::distribution::{EditDistributionModel, NewDistributionModel};
use anyhow::Error;
use rainbow_common::dcat_formats::DctFormats;
use thiserror::Error;
use urn::Urn;

#[async_trait::async_trait]
pub trait DistributionRepositoryTrait: Send + Sync {
    async fn get_all_distributions(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<distribution::Model>, DistributionRepoErrors>;
    async fn get_batch_distributions(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<distribution::Model>, DistributionRepoErrors>;

    async fn get_distributions_by_dataset_id(
        &self,
        dataset_id: &Urn,
    ) -> anyhow::Result<Vec<distribution::Model>, DistributionRepoErrors>;
    async fn get_distribution_by_dataset_id_and_dct_format(
        &self,
        dataset_id: &Urn,
        dct_formats: &DctFormats,
    ) -> anyhow::Result<distribution::Model, DistributionRepoErrors>;
    async fn get_distribution_by_id(
        &self,
        distribution_id: &Urn,
    ) -> anyhow::Result<Option<distribution::Model>, DistributionRepoErrors>;
    async fn put_distribution_by_id(
        &self,
        distribution_id: &Urn,
        edit_distribution_model: &EditDistributionModel,
    ) -> anyhow::Result<distribution::Model, DistributionRepoErrors>;
    async fn create_distribution(
        &self,
        new_distribution_model: &NewDistributionModel,
    ) -> anyhow::Result<distribution::Model, DistributionRepoErrors>;
    async fn delete_distribution_by_id(&self, distribution_id: &Urn) -> anyhow::Result<(), DistributionRepoErrors>;
}

#[derive(Error, Debug)]
pub enum DistributionRepoErrors {
    #[error("Distribution not found")]
    DistributionNotFound,
    #[error("Error fetching distribution. {0}")]
    ErrorFetchingDistribution(Error),
    #[error("Error creating distribution. {0}")]
    ErrorCreatingDistribution(Error),
    #[error("Error deleting distribution. {0}")]
    ErrorDeletingDistribution(Error),
    #[error("Error updating distribution. {0}")]
    ErrorUpdatingDistribution(Error),
}
