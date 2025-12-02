use crate::data::entities::negotiation_process;
use crate::data::entities::negotiation_process::{EditNegotiationProcessModel, NewNegotiationProcessModel};
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[async_trait::async_trait]
pub trait NegotiationProcessRepoTrait: Send + Sync {
    async fn get_all_negotiation_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<negotiation_process::Model>, NegotiationProcessRepoErrors>;
    async fn get_batch_negotiation_processes(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<negotiation_process::Model>, NegotiationProcessRepoErrors>;
    async fn get_negotiation_process_by_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<negotiation_process::Model>, NegotiationProcessRepoErrors>;
    async fn get_negotiation_process_by_key_id(
        &self,
        key_id: &str,
        id: &Urn,
    ) -> anyhow::Result<Option<negotiation_process::Model>, NegotiationProcessRepoErrors>;
    async fn get_negotiation_process_by_key_value(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<negotiation_process::Model>, NegotiationProcessRepoErrors>;
    async fn create_negotiation_process(
        &self,
        new_model: &NewNegotiationProcessModel,
    ) -> anyhow::Result<negotiation_process::Model, NegotiationProcessRepoErrors>;
    async fn put_negotiation_process(
        &self,
        id: &Urn,
        edit_model: &EditNegotiationProcessModel,
    ) -> anyhow::Result<negotiation_process::Model, NegotiationProcessRepoErrors>;
    async fn delete_negotiation_process(&self, id: &Urn) -> anyhow::Result<(), NegotiationProcessRepoErrors>;
}

#[derive(Debug, Error)]
pub enum NegotiationProcessRepoErrors {
    #[error("Negotiation Process not found")]
    NegotiationProcessNotFound,
    #[error("Error fetching negotiation process. {0}")]
    ErrorFetchingNegotiationProcess(Error),
    #[error("Error creating negotiation process. {0}")]
    ErrorCreatingNegotiationProcess(Error),
    #[error("Error deleting negotiation process. {0}")]
    ErrorDeletingNegotiationProcess(Error),
    #[error("Error updating negotiation process. {0}")]
    ErrorUpdatingNegotiationProcess(Error),
}
