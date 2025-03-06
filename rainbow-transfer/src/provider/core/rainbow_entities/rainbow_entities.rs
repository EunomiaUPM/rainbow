use crate::provider::core::rainbow_entities::rainbow_err::RainbowTransferProviderErrors;
use crate::provider::core::rainbow_entities::RainbowTransferProviderServiceTrait;
use axum::async_trait;
use rainbow_db::transfer_provider::entities::{transfer_message, transfer_process};
use rainbow_db::transfer_provider::repo::TransferProviderRepoFactory;
use std::sync::Arc;
use urn::Urn;

pub struct RainbowTransferProviderServiceImpl<T>
where
    T: TransferProviderRepoFactory + Send + Sync,
{
    repo: Arc<T>,
}

impl<T> RainbowTransferProviderServiceImpl<T>
where
    T: TransferProviderRepoFactory + Send + Sync,
{
    pub fn new(repo: Arc<T>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T> RainbowTransferProviderServiceTrait for RainbowTransferProviderServiceImpl<T>
where
    T: TransferProviderRepoFactory + Send + Sync,
{
    async fn get_all_transfers(&self) -> anyhow::Result<Vec<transfer_process::Model>> {
        let transfer_processes = self.repo
            .get_all_transfer_processes(None, None)
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?;
        Ok(transfer_processes)
    }

    async fn get_transfer_by_id(
        &self,
        provider_pid: Urn,
    ) -> anyhow::Result<transfer_process::Model> {
        let transfer_processes = self.repo
            .get_transfer_process_by_provider(provider_pid.clone())
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?
            .ok_or(RainbowTransferProviderErrors::ProcessNotFound {
                provider_pid: Option::from(provider_pid),
                consumer_pid: None,
            })?;

        Ok(transfer_processes)
    }

    async fn get_transfer_by_consumer_id(&self, consumer_id: Urn) -> anyhow::Result<transfer_process::Model> {
        let transfer_processes = self.repo
            .get_transfer_process_by_consumer(consumer_id.clone())
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?
            .ok_or(RainbowTransferProviderErrors::ProcessNotFound {
                provider_pid: None,
                consumer_pid: Option::from(consumer_id),
            })?;
        Ok(transfer_processes)
    }

    async fn get_messages_by_transfer(
        &self,
        transfer_id: Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>> {
        let messages = self.repo
            .get_all_transfer_messages_by_provider(transfer_id)
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?;
        Ok(messages)
    }

    async fn get_messages_by_id(
        &self,
        transfer_id: Urn,
        message_id: Urn,
    ) -> anyhow::Result<transfer_message::Model> {
        let message = self.repo
            .get_transfer_message_by_id(message_id.clone())
            .await
            .map_err(RainbowTransferProviderErrors::DbErr)?
            .ok_or(RainbowTransferProviderErrors::MessageNotFound {
                transfer_id: Option::from(transfer_id),
                message_id: Option::from(message_id),
            })?;

        Ok(message)
    }
}
