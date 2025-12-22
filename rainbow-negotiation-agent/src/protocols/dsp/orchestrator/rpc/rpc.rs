/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use crate::entities::negotiation_process::NegotiationProcessDto;
use crate::protocols::dsp::orchestrator::rpc::RPCOrchestratorTrait;
use crate::protocols::dsp::orchestrator::rpc::peer_communication::PeerCommunication;
use crate::protocols::dsp::orchestrator::rpc::persistence::OrchestrationPersistenceForRpc;
use crate::protocols::dsp::orchestrator::rpc::types::{
    RpcNegotiationAgreementMessageDto, RpcNegotiationEventAcceptedMessageDto, RpcNegotiationEventFinalizedMessageDto,
    RpcNegotiationMessageDto, RpcNegotiationOfferInitMessageDto, RpcNegotiationOfferMessageDto,
    RpcNegotiationProcessMessageTrait, RpcNegotiationRequestInitMessageDto, RpcNegotiationRequestMessageDto,
    RpcNegotiationTerminationMessageDto, RpcNegotiationVerificationMessageDto,
};
use crate::protocols::dsp::orchestrator::traits::orchestration_helpers::OrchestrationHelpers;
use crate::protocols::dsp::persistence::NegotiationPersistenceTrait;
use crate::protocols::dsp::protocol_types::{
    NegotiationAckMessageDto, NegotiationAgreementMessageDto, NegotiationEventMessageDto, NegotiationEventType,
    NegotiationOfferInitMessageDto, NegotiationOfferMessageDto, NegotiationProcessMessageTrait,
    NegotiationProcessMessageWrapper, NegotiationRequestInitMessageDto, NegotiationRequestMessageDto,
    NegotiationTerminationMessageDto, NegotiationVerificationMessageDto,
};
use crate::protocols::dsp::validator::traits::validation_rpc_steps::ValidationRpcSteps;
use rainbow_common::config::services::ContractsConfig;
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::dsp_common::context_field::ContextField;
use rainbow_common::dsp_common::odrl::{OdrlAgreement, OdrlMessageOffer, OdrlTypes};
use rainbow_common::http_client::HttpClient;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;

#[allow(unused)]
pub struct RPCOrchestratorService {
    validator: Arc<dyn ValidationRpcSteps>,
    persistence_service: Arc<OrchestrationPersistenceForRpc>,
    _config: Arc<ContractsConfig>,
    http_client: Arc<HttpClient>,
}

impl RPCOrchestratorService {
    pub fn new(
        validator: Arc<dyn ValidationRpcSteps>,
        persistence_service: Arc<OrchestrationPersistenceForRpc>,
        _config: Arc<ContractsConfig>,
        http_client: Arc<HttpClient>,
    ) -> RPCOrchestratorService {
        RPCOrchestratorService { validator, persistence_service, _config, http_client }
    }
}

impl OrchestrationHelpers for RPCOrchestratorService {}

#[async_trait::async_trait]
impl RPCOrchestratorTrait for RPCOrchestratorService {
    async fn setup_negotiation_request_init_rpc(
        &self,
        input: &RpcNegotiationRequestInitMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationRequestInitMessageDto>> {
        self.validator.negotiation_request_init_rpc(input).await?;

        // send to peer
        let provider_address = self.get_rpc_provider_address_safely(input)?;
        let peer_url = format!("{}/negotiations/request", provider_address);
        let request_body: NegotiationProcessMessageWrapper<NegotiationRequestInitMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process = self.persistence_service.create_new(input, &request_body.dto, &response.dto).await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)
    }

    async fn setup_negotiation_request_rpc(
        &self,
        input: &RpcNegotiationRequestMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationRequestMessageDto>> {
        self.validator.negotiation_request_rpc(input).await?;

        // extract fields
        let id = self.get_rpc_consumer_pid_safely(input)?.to_string();
        let current_process = self.persistence_service.fetch_process(id.as_str()).await?;
        let role = !current_process.inner.role.parse::<RoleConfig>()?;
        let role_identifier = self.parse_role_into_identifier(&role)?.to_string();
        let identifier = current_process.identifiers.get(&role_identifier).unwrap();
        let peer_address = current_process.inner.callback_address.unwrap();

        // send to peer
        let peer_url = format!("{}/negotiations/{}/request", peer_address, identifier);
        let request_body: NegotiationProcessMessageWrapper<NegotiationRequestMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process =
            self.persistence_service.update_with_offer(id.as_str(), input, &request_body.dto, &response.dto).await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)
    }

    async fn setup_negotiation_offer_init_rpc(
        &self,
        input: &RpcNegotiationOfferInitMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationOfferInitMessageDto>> {
        self.validator.negotiation_offer_init_rpc(input).await?;

        // send to peer
        let provider_address = self.get_rpc_provider_address_safely(input)?;
        let peer_url = format!("{}/negotiations/offers", provider_address);
        let request_body: NegotiationProcessMessageWrapper<NegotiationOfferInitMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process = self.persistence_service.create_new(input, &request_body.dto, &response.dto).await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)
    }

    async fn setup_negotiation_offer_rpc(
        &self,
        input: &RpcNegotiationOfferMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationOfferMessageDto>> {
        self.validator.negotiation_offer_rpc(input).await?;

        // extract fields
        let id = self.get_rpc_consumer_pid_safely(input)?.to_string();
        let current_process = self.persistence_service.fetch_process(id.as_str()).await?;
        let role = !current_process.inner.role.parse::<RoleConfig>()?;
        let role_identifier = self.parse_role_into_identifier(&role)?.to_string();
        let identifier = current_process.identifiers.get(&role_identifier).unwrap();
        let peer_address = current_process.inner.callback_address.unwrap();

        // send to peer
        let peer_url = format!("{}/negotiations/{}/offers", peer_address, identifier);
        let request_body: NegotiationProcessMessageWrapper<NegotiationOfferMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process =
            self.persistence_service.update_with_offer(id.as_str(), input, &request_body.dto, &response.dto).await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)
    }

    async fn setup_negotiation_agreement_rpc(
        &self,
        input: &RpcNegotiationAgreementMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationAgreementMessageDto>> {
        self.validator.negotiation_agreement_rpc(input).await?;
        // extract fields
        let id = self.get_rpc_consumer_pid_safely(input)?.to_string();
        let current_process = self.persistence_service.fetch_process(id.as_str()).await?;
        let role = !current_process.inner.role.parse::<RoleConfig>()?;
        let role_identifier = self.parse_role_into_identifier(&role)?.to_string();
        let identifier = current_process.identifiers.get(&role_identifier).unwrap();
        let peer_address = current_process.inner.callback_address.unwrap();

        // get last offer
        let last_offer =
            self.persistence_service.fetch_last_offer_by_process(current_process.inner.id.as_str()).await?;
        let offer = serde_json::from_value::<OdrlMessageOffer>(last_offer.inner.offer_content)?;

        // send to peer
        let peer_url = format!("{}/negotiations/{}/agreement", peer_address, identifier);
        let mut request_body: NegotiationProcessMessageWrapper<NegotiationAgreementMessageDto> = input.clone().into();
        request_body.dto.agreement = OdrlAgreement {
            id: self.create_entity_urn("agreement")?,
            profile: offer.profile,
            permission: offer.permission,
            obligation: offer.obligation,
            _type: OdrlTypes::Agreement,
            target: offer.target,
            assigner: "".to_string(),
            assignee: "".to_string(),
            timestamp: Some(chrono::Utc::now().timestamp().to_string()),
            prohibition: offer.prohibition,
        };
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process = self
            .persistence_service
            .update_with_new_agreement(id.as_str(), input, &request_body.dto, &response.dto)
            .await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)
    }

    async fn setup_negotiation_agreement_verification_rpc(
        &self,
        input: &RpcNegotiationVerificationMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationVerificationMessageDto>> {
        self.validator.negotiation_agreement_verification_rpc(input).await?;

        // extract fields
        let id = self.get_rpc_consumer_pid_safely(input)?.to_string();
        let current_process = self.persistence_service.fetch_process(id.as_str()).await?;
        let role = !current_process.inner.role.parse::<RoleConfig>()?;
        let role_identifier = self.parse_role_into_identifier(&role)?.to_string();
        let identifier = current_process.identifiers.get(&role_identifier).unwrap();
        let peer_address = current_process.inner.callback_address.unwrap();

        // send to peer
        let peer_url = format!(
            "{}/negotiations/{}/agreement/verification",
            peer_address, identifier
        );
        let request_body: NegotiationProcessMessageWrapper<NegotiationVerificationMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process = self
            .persistence_service
            .update_with_agreement(id.as_str(), input, &request_body.dto, &response.dto)
            .await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)
    }

    async fn setup_negotiation_event_accepted_rpc(
        &self,
        input: &RpcNegotiationEventAcceptedMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationEventAcceptedMessageDto>> {
        self.validator.negotiation_event_accepted_rpc(input).await?;

        // extract fields
        let id = self.get_rpc_consumer_pid_safely(input)?.to_string();
        let current_process = self.persistence_service.fetch_process(id.as_str()).await?;
        let role = !current_process.inner.role.parse::<RoleConfig>()?;
        let role_identifier = self.parse_role_into_identifier(&role)?.to_string();
        let identifier = current_process.identifiers.get(&role_identifier).unwrap();
        let peer_address = current_process.inner.callback_address.unwrap();

        // send to peer
        let peer_url = format!("{}/negotiations/{}/events", peer_address, identifier);
        let request_body: NegotiationProcessMessageWrapper<NegotiationEventMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process =
            self.persistence_service.update(id.as_str(), input, &request_body.dto, &response.dto).await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)
    }

    async fn setup_negotiation_event_finalized_rpc(
        &self,
        input: &RpcNegotiationEventFinalizedMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationEventFinalizedMessageDto>> {
        self.validator.negotiation_event_finalized_rpc(input).await?;

        // extract fields
        let id = self.get_rpc_consumer_pid_safely(input)?.to_string();
        let current_process = self.persistence_service.fetch_process(id.as_str()).await?;
        let role = !current_process.inner.role.parse::<RoleConfig>()?;
        let role_identifier = self.parse_role_into_identifier(&role)?.to_string();
        let identifier = current_process.identifiers.get(&role_identifier).unwrap();
        let peer_address = current_process.inner.callback_address.unwrap();

        // send to peer
        let peer_url = format!("{}/negotiations/{}/events", peer_address, identifier);
        let request_body: NegotiationProcessMessageWrapper<NegotiationEventMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process = self
            .persistence_service
            .update_with_agreement(id.as_str(), input, &request_body.dto, &response.dto)
            .await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)
    }

    async fn setup_negotiation_termination_rpc(
        &self,
        input: &RpcNegotiationTerminationMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationTerminationMessageDto>> {
        self.validator.negotiation_termination_rpc(input).await?;

        // extract fields
        let id = self.get_rpc_consumer_pid_safely(input)?.to_string();
        let current_process = self.persistence_service.fetch_process(id.as_str()).await?;
        let role = !current_process.inner.role.parse::<RoleConfig>()?;
        let role_identifier = self.parse_role_into_identifier(&role)?.to_string();
        let identifier = current_process.identifiers.get(&role_identifier).unwrap();
        let peer_address = current_process.inner.callback_address.unwrap();

        // send to peer
        let peer_url = format!("{}/negotiations/{}/termination", peer_address, identifier);
        let request_body: NegotiationProcessMessageWrapper<NegotiationTerminationMessageDto> = input.clone().into();
        self.http_client.set_auth_token("blabla".to_string()).await;
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process =
            self.persistence_service.update(id.as_str(), input, &request_body.dto, &response.dto).await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)
    }
}
