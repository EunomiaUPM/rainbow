// use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatahubDomain};
use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDomain};
use crate::core::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatasetBasicInfo};
use axum::async_trait;

pub mod datahub_proxy;
pub mod datahub_proxy_types;

#[mockall::automock]
#[async_trait]
pub trait DatahubProxyTrait: Send + Sync + 'static {
    async fn get_datahub_domains(&self) -> anyhow::Result<Vec<DatahubDomain>>;
    async fn get_datahub_datasets_by_domain_id(&self, id: String) -> anyhow::Result<Vec<DatasetBasicInfo>>;
    async fn get_datahub_dataset_by_id(&self, id: String) -> anyhow::Result<DatahubDataset>;
    // async fn get_dataset_policies(&self, id: String) -> anyhow::Result<DatahubDataset>;
    // async fn add_policy_to_dataset(&self, dataset_urn: String, property_name: String, property_value: String) -> anyhow::Result<bool> ;
}