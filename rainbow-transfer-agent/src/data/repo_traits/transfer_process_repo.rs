use crate::data::entities::transfer_process;
use crate::data::entities::transfer_process::{EditTransferProcessModel, NewTransferProcessModel};
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[mockall::automock]
#[async_trait::async_trait]
pub trait TransferProcessRepoTrait: Send + Sync {
    async fn get_all_transfer_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_process::Model>, TransferProcessRepoErrors>;
    async fn get_batch_transfer_processes(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<transfer_process::Model>, TransferProcessRepoErrors>;
    async fn get_transfer_process_by_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<transfer_process::Model>, TransferProcessRepoErrors>;
    async fn get_transfer_process_by_key_id(
        &self,
        key_id: &str,
        id: &Urn,
    ) -> anyhow::Result<Option<transfer_process::Model>, TransferProcessRepoErrors>;
    async fn get_transfer_process_by_key_value(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<transfer_process::Model>, TransferProcessRepoErrors>;
    async fn create_transfer_process(
        &self,
        new_model: &NewTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model, TransferProcessRepoErrors>;
    async fn put_transfer_process(
        &self,
        id: &Urn,
        edit_model: &EditTransferProcessModel,
    ) -> anyhow::Result<transfer_process::Model, TransferProcessRepoErrors>;
    async fn delete_transfer_process(&self, id: &Urn) -> anyhow::Result<(), TransferProcessRepoErrors>;
}

#[derive(Debug, Error)]
pub enum TransferProcessRepoErrors {
    #[error("Transfer Process not found")]
    TransferProcessNotFound,
    #[error("Error fetching transfer process. {0}")]
    ErrorFetchingTransferProcess(Error),
    #[error("Error creating transfer process. {0}")]
    ErrorCreatingTransferProcess(Error),
    #[error("Error deleting transfer process. {0}")]
    ErrorDeletingTransferProcess(Error),
    #[error("Error updating transfer process. {0}")]
    ErrorUpdatingTransferProcess(Error),
}
