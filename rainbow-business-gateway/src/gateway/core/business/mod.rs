use axum::async_trait;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::protocol::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatahubDomain};
use rainbow_db::datahub::entities::policy_templates;
use urn::Urn;

pub mod business;

#[async_trait]
pub trait BusinessCatalogTrait: Send + Sync + 'static {
    async fn get_catalogs(&self, token: String) -> anyhow::Result<Vec<DatahubDomain>>;
    async fn get_datasets_by_catalog(&self, catalog_id: Urn, token: String) -> anyhow::Result<Vec<DatahubDataset>>;
    async fn get_dataset(&self, dataset_id: Urn, token: String) -> anyhow::Result<DatahubDataset>;
    async fn get_policy_templates(&self, token: String) -> anyhow::Result<Vec<policy_templates::Model>>;
    async fn get_policy_template_by_id(&self, template_id: String, token: String) -> anyhow::Result<policy_templates::Model>;
    async fn get_policy_offers_by_dataset(&self, dataset_id: Urn, token: String) -> anyhow::Result<Vec<OdrlOffer>>;
    async fn post_policy_offer(&self, dataset_id: Urn, odrl_offer: OdrlOffer, token: String) -> anyhow::Result<OdrlOffer>;
    async fn delete_policy_offer(&self, dataset_id: Urn, policy_id: Urn, token: String) -> anyhow::Result<()>;
}