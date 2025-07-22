/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::gateway::http::business_router_types::{RainbowBusinessAcceptanceRequest, RainbowBusinessNegotiationRequest, RainbowBusinessTerminationRequest};
use axum::async_trait;
use rainbow_common::auth::business::RainbowBusinessLoginRequest;
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
    async fn get_policy_template_by_id(
        &self,
        template_id: String,
        token: String,
    ) -> anyhow::Result<policy_templates::Model>;
    async fn get_policy_offers_by_dataset(&self, dataset_id: Urn, token: String) -> anyhow::Result<Vec<OdrlOffer>>;
    async fn post_policy_offer(
        &self,
        dataset_id: Urn,
        odrl_offer: OdrlPolicyInfo,
        token: String,
    ) -> anyhow::Result<OdrlOffer>;
    async fn delete_policy_offer(&self, dataset_id: Urn, policy_id: Urn, token: String) -> anyhow::Result<()>;
    async fn get_business_negotiation_requests(&self, token: String) -> anyhow::Result<Value>;
    async fn get_business_negotiation_request_by_id(
        &self,
        request_id: Urn,
        token: String,
    ) -> anyhow::Result<ContractAckMessage>;
    async fn get_consumer_negotiation_requests(&self, participant_id: String, token: String) -> anyhow::Result<Value>;
    async fn get_consumer_negotiation_request_by_id(
        &self,
        participant_id: String,
        request_id: Urn,
        token: String,
    ) -> anyhow::Result<ContractAckMessage>;
    async fn accept_request(&self, input: RainbowBusinessAcceptanceRequest, token: String) -> anyhow::Result<Value>;
    async fn terminate_request(&self, input: RainbowBusinessTerminationRequest, token: String) -> anyhow::Result<Value>;
    async fn create_request(&self, input: RainbowBusinessNegotiationRequest, token: String) -> anyhow::Result<Value>;
    async fn login(&self, input: RainbowBusinessLoginRequest) -> anyhow::Result<String>;
    async fn login_poll(&self, input: RainbowBusinessLoginRequest) -> anyhow::Result<Value>;
}
