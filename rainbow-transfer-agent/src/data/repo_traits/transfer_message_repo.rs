use crate::data::entities::transfer_message;
use crate::data::entities::transfer_message::NewTransferMessageModel;
use anyhow::Error;
use thiserror::Error;
use urn::Urn;

#[mockall::automock]
#[async_trait::async_trait]
pub trait TransferMessageRepoTrait: Send + Sync {
    // Obtener todos (paginado)
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferMessageRepoErrors>;

    async fn get_messages_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferMessageRepoErrors>;

    async fn get_transfer_message_by_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferMessageRepoErrors>;

    async fn create_transfer_message(
        &self,
        new_model: &NewTransferMessageModel,
    ) -> anyhow::Result<transfer_message::Model, TransferMessageRepoErrors>;

    async fn delete_transfer_message(&self, id: &Urn) -> anyhow::Result<(), TransferMessageRepoErrors>;
}

#[derive(Debug, Error)]
pub enum TransferMessageRepoErrors {
    #[error("Transfer Message not found")]
    TransferMessageNotFound,
    #[error("Error fetching transfer message. {0}")]
    ErrorFetchingTransferMessage(Error),
    #[error("Error creating transfer message. {0}")]
    ErrorCreatingTransferMessage(Error),
    #[error("Error deleting transfer message. {0}")]
    ErrorDeletingTransferMessage(Error),
}
