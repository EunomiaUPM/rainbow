use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatahubDomain};
use axum::async_trait;

pub mod datahub_proxy;
pub mod datahub_proxy_types;

#[mockall::automock]
#[async_trait]
pub trait DatahubProxyTrait: Send + Sync {
    async fn get_datahub_domains(&self) -> anyhow::Result<Vec<DatahubDomain>>;
    async fn get_datahub_domain_by_id(&self, id: String) -> anyhow::Result<DatahubDomain>;
    async fn get_datahub_datasets_by_domain_id(&self, id: String) -> anyhow::Result<Vec<DatahubDataset>>;
    async fn get_datahub_dataset_by_id(&self, id: String) -> anyhow::Result<DatahubDataset>;
}