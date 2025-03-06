use axum::async_trait;
use rainbow_db::transfer_provider::entities::{transfer_message, transfer_process};
use rainbow_db::transfer_provider::repo::TransferProviderRepoFactory;
use urn::Urn;

pub mod rainbow_entities;
pub mod rainbow_err;

#[mockall::automock]
#[async_trait]
pub trait RainbowTransferProviderServiceTrait: Send + Sync {
    async fn get_all_transfers(&self) -> anyhow::Result<Vec<transfer_process::Model>>;
    async fn get_transfer_by_id(
        &self,
        provider_pid: Urn,
    ) -> anyhow::Result<transfer_process::Model>;
    async fn get_transfer_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<transfer_process::Model>;
    async fn get_messages_by_transfer(
        &self,
        transfer_id: Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>>;
    async fn get_messages_by_id(
        &self,
        transfer_id: Urn,
        message_id: Urn,
    ) -> anyhow::Result<transfer_message::Model>;
}