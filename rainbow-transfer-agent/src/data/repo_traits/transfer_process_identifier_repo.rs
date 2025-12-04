
use crate::data::entities::transfer_process_identifier;
use crate::data::entities::transfer_process_identifier::{EditTransferIdentifierModel, NewTransferIdentifierModel};
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[mockall::automock]
#[async_trait::async_trait]
#[allow(unused)]
pub trait TransferIdentifierRepoTrait: Send + Sync {
    async fn get_all_identifiers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_process_identifier::Model>, TransferIdentifierRepoErrors>;

    async fn get_identifiers_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<transfer_process_identifier::Model>, TransferIdentifierRepoErrors>;

    async fn get_identifier_by_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<transfer_process_identifier::Model>, TransferIdentifierRepoErrors>;

    async fn get_identifier_by_key(
        &self,
        process_id: &Urn,
        key: &str,
    ) -> anyhow::Result<Option<transfer_process_identifier::Model>, TransferIdentifierRepoErrors>;

    async fn create_identifier(
        &self,
        new_model: &NewTransferIdentifierModel,
    ) -> anyhow::Result<transfer_process_identifier::Model, TransferIdentifierRepoErrors>;

    async fn put_identifier(
        &self,
        id: &Urn,
        edit_model: &EditTransferIdentifierModel,
    ) -> anyhow::Result<transfer_process_identifier::Model, TransferIdentifierRepoErrors>;

    async fn delete_identifier(&self, id: &Urn) -> anyhow::Result<(), TransferIdentifierRepoErrors>;
}

#[derive(Debug, Error)]
pub enum TransferIdentifierRepoErrors {
    #[error("Transfer Identifier not found")]
    TransferIdentifierNotFound,
    #[error("Error fetching transfer identifier. {0}")]
    ErrorFetchingTransferIdentifier(Error),
    #[error("Error creating transfer identifier. {0}")]
    ErrorCreatingTransferIdentifier(Error),
    #[error("Error deleting transfer identifier. {0}")]
    ErrorDeletingTransferIdentifier(Error),
    #[error("Error updating transfer identifier. {0}")]
    ErrorUpdatingTransferIdentifier(Error),
}
