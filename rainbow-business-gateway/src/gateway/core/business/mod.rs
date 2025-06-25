use crate::gateway::http::business_router_types::RainbowBusinessNegotiationRequest;
use axum::async_trait;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_odrl::{OdrlOffer, OdrlPolicyInfo};
use rainbow_common::protocol::datahub_proxy::datahub_proxy_types::{DatahubDataset, DatahubDomain};
use rainbow_db::datahub::entities::policy_templates;
use serde_json::Value;
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
    async fn post_policy_offer(&self, dataset_id: Urn, odrl_offer: OdrlPolicyInfo, token: String) -> anyhow::Result<OdrlOffer>;
    async fn delete_policy_offer(&self, dataset_id: Urn, policy_id: Urn, token: String) -> anyhow::Result<()>;
    async fn get_business_negotiation_requests(&self, token: String) -> anyhow::Result<Vec<ContractAckMessage>>;
    async fn get_business_negotiation_request_by_id(&self, request_id: Urn, token: String) -> anyhow::Result<ContractAckMessage>;
    async fn get_consumer_negotiation_requests(&self, token: String) -> anyhow::Result<Vec<ContractAckMessage>>;
    async fn get_consumer_negotiation_request_by_id(&self, request_id: Urn, token: String) -> anyhow::Result<ContractAckMessage>;
    async fn accept_request(&self, request_id: Urn, token: String) -> anyhow::Result<()>;
    async fn create_request(&self, input: RainbowBusinessNegotiationRequest, token: String) -> anyhow::Result<Value>;
}