use std::sync::Arc;
use tracing::error;
use urn::Urn;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use crate::db::entities::transfer_message::NewTransferMessageModel;
use crate::db::factory_trait::TransferAgentRepoTrait;
use crate::db::repo_traits::transfer_message_repo::TransferMessageRepoErrors;
use crate::entities::transfer_messages::{NewTransferMessageDto, TransferAgentMessagesTrait, TransferMessageDto};

pub struct TransferAgentMessagesService {
    pub transfer_repo: Arc<dyn TransferAgentRepoTrait>,
}

impl TransferAgentMessagesService {
    pub fn new(transfer_repo: Arc<dyn TransferAgentRepoTrait>) -> Self {
        Self { transfer_repo }
    }
}

#[async_trait::async_trait]
impl TransferAgentMessagesTrait for TransferAgentMessagesService {
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<TransferMessageDto>> {
        let messages = self
            .transfer_repo
            .get_transfer_message_repo() // Asumo que existe este método en el Factory
            .get_all_transfer_messages(limit, page)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        Ok(messages
            .into_iter()
            .map(|m| TransferMessageDto { inner: m })
            .collect())
    }

    async fn get_messages_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<TransferMessageDto>> {
        let messages = self
            .transfer_repo
            .get_transfer_message_repo()
            .get_messages_by_process_id(process_id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        Ok(messages
            .into_iter()
            .map(|m| TransferMessageDto { inner: m })
            .collect())
    }

    async fn get_transfer_message_by_id(&self, id: &Urn) -> anyhow::Result<TransferMessageDto> {
        let message = self
            .transfer_repo
            .get_transfer_message_repo()
            .get_transfer_message_by_id(id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?
            .ok_or_else(|| {
                let err = CommonErrors::missing_resource_new(
                    &id.to_string(),
                    "Transfer Message not found"
                );
                error!("{}", err.log());
                err
            })?;

        Ok(TransferMessageDto { inner: message })
    }

    async fn create_transfer_message(
        &self,
        new_model_dto: &NewTransferMessageDto,
    ) -> anyhow::Result<TransferMessageDto> {
        let new_model: NewTransferMessageModel = new_model_dto.clone().into();

        let created = self
            .transfer_repo
            .get_transfer_message_repo()
            .create_transfer_message(&new_model)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        Ok(TransferMessageDto { inner: created })
    }

    async fn delete_transfer_message(&self, id: &Urn) -> anyhow::Result<()> {
        self.transfer_repo
            .get_transfer_message_repo()
            .delete_transfer_message(id)
            .await
            .map_err(|e| {
                match e {
                    TransferMessageRepoErrors::TransferMessageNotFound => {
                        let err = CommonErrors::missing_resource_new(
                            &id.to_string(),
                            "Transfer Message not found for deletion"
                        );
                        error!("{}", err.log());
                        err
                    },
                    _ => {
                        let err = CommonErrors::database_new(&e.to_string());
                        error!("{}", err.log());
                        err
                    }
                }
            })?;
        Ok(())
    }
}