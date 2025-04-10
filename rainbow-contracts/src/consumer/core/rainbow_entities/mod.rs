use crate::consumer::core::rainbow_entities::rainbow_entities_types::{
    EditContractNegotiationRequest, NewContractNegotiationRequest,
};
use axum::async_trait;
use rainbow_db::contracts_consumer::entities::cn_process;
use urn::Urn;

pub mod rainbow_entities;
pub mod rainbow_entities_errors;
pub mod rainbow_entities_types;

#[mockall::automock]
#[async_trait]
pub trait RainbowEntitiesContractNegotiationConsumerTrait: Send + Sync {
    async fn get_cn_processes(&self) -> anyhow::Result<Vec<cn_process::Model>>;
    async fn get_cn_process_by_id(&self, process_id: Urn) -> anyhow::Result<cn_process::Model>;
    async fn get_cn_process_by_provider(&self, provider_id: Urn) -> anyhow::Result<cn_process::Model>;
    async fn get_cn_process_by_consumer(&self, consumer_id: Urn) -> anyhow::Result<cn_process::Model>;
    async fn post_cn_process(&self, input: NewContractNegotiationRequest) -> anyhow::Result<cn_process::Model>;
    async fn put_cn_process(
        &self,
        process_id: Urn,
        input: EditContractNegotiationRequest,
    ) -> anyhow::Result<cn_process::Model>;
    async fn delete_cn_process(&self, process_id: Urn) -> anyhow::Result<()>;
}
