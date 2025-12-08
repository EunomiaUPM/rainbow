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
    NegotiationOfferMessageDto, NegotiationProcessMessageTrait, NegotiationProcessMessageWrapper,
    NegotiationRequestInitMessageDto, NegotiationRequestMessageDto, NegotiationTerminationMessageDto,
    NegotiationVerificationMessageDto,
};
use crate::protocols::dsp::validator::traits::validation_rpc_steps::ValidationRpcSteps;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use rainbow_common::http_client::HttpClient;
use rainbow_common::protocol::context_field::ContextField;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;

#[allow(unused)]
pub struct RPCOrchestratorService {
    validator: Arc<dyn ValidationRpcSteps>,
    persistence_service: Arc<OrchestrationPersistenceForRpc>,
    _config: Arc<ApplicationGlobalConfig>,
    http_client: Arc<HttpClient>,
}

impl RPCOrchestratorService {
    pub fn new(
        validator: Arc<dyn ValidationRpcSteps>,
        persistence_service: Arc<OrchestrationPersistenceForRpc>,
        _config: Arc<ApplicationGlobalConfig>,
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
        dbg!(&request_body);
        let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
            self.http_client.post_json(peer_url.as_str(), &request_body).await?;

        // persist
        let negotiation_process = self.persistence_service.create_new(input, &response.dto).await?;

        let response =
            RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        Ok(response)

        // // get from input
        // let request_body: NegotiationProcessMessageWrapper<NegotiationRequestInitMessageDto> = input.clone().into();
        // let provider_address =
        //     input.get_provider_address().ok_or_else(|| anyhow::anyhow!("No provider address found"))?;
        // // create url
        // let peer_url = format!("{}/negotiations/request", provider_address);
        // let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
        //     self.http_client.post_json(peer_url.as_str(), &request_body).await?;
        // // persist
        // let negotiation_process = self
        //     .persistence_service
        //     .create_new(input)
        //     .await?;
        //
        // let response =
        //     RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        // Ok(response)
    }

    async fn setup_negotiation_request_rpc(
        &self,
        input: &RpcNegotiationRequestMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationRequestMessageDto>> {
        // self.validator.negotiation_request_rpc(input).await?;
        // // get from input
        // let input_negotiation_id = input.get_consumer_pid().unwrap();
        // // fetch current process
        // let negotiation_process =
        //     self.persistence_service.fetch_process(input_negotiation_id.to_string().as_str()).await?;
        // let provider_pid = negotiation_process.identifiers.get("providerPid").unwrap();
        // let consumer_pid = negotiation_process.identifiers.get("consumerPid").unwrap();
        // // create message
        // let offer = input.get_offer().unwrap();
        // let negotiation_process_into_trait = NegotiationRequestMessageDto {
        //     provider_pid: Urn::from_str(provider_pid.as_str())?,
        //     consumer_pid: Urn::from_str(consumer_pid.as_str())?,
        //     offer,
        // };
        // // get uri
        // let identifier_key = match negotiation_process.inner.role.as_str() {
        //     "Provider" => "consumerPid",
        //     "Consumer" => "providerPid",
        //     _ => "providerPid",
        // };
        // let peer_url_id = negotiation_process.identifiers.get(identifier_key).unwrap();
        // // facades
        // // validate, send and persist
        // let (response, negotiation_process) = self
        //     .validate_and_send(
        //         &negotiation_process,
        //         Arc::new(negotiation_process_into_trait.clone()),
        //         peer_url_id,
        //         "request",
        //     )
        //     .await?;
        // // bye!
        // let response =
        //     RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        // Ok(response)
        todo!()
    }

    async fn setup_negotiation_offer_init_rpc(
        &self,
        input: &RpcNegotiationOfferInitMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationOfferInitMessageDto>> {
        todo!()
    }

    async fn setup_negotiation_offer_rpc(
        &self,
        input: &RpcNegotiationOfferMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationOfferMessageDto>> {
        // self.validator.negotiation_offer_rpc(input).await?;
        // // get from input
        // let input_negotiation_id = input.get_consumer_pid().unwrap();
        // // fetch current process
        // let negotiation_process =
        //     self.persistence_service.fetch_process(input_negotiation_id.to_string().as_str()).await?;
        // let provider_pid = negotiation_process.identifiers.get("providerPid").unwrap();
        // let consumer_pid = negotiation_process.identifiers.get("consumerPid").unwrap();
        // // create message
        // let offer = input.get_offer().unwrap();
        // let negotiation_process_into_trait = NegotiationOfferMessageDto {
        //     provider_pid: Urn::from_str(provider_pid.as_str())?,
        //     consumer_pid: Urn::from_str(consumer_pid.as_str())?,
        //     offer,
        //     callback_address: None,
        // };
        // // get uri
        // let identifier_key = match negotiation_process.inner.role.as_str() {
        //     "Provider" => "consumerPid",
        //     "Consumer" => "providerPid",
        //     _ => "providerPid",
        // };
        // let peer_url_id = negotiation_process.identifiers.get(identifier_key).unwrap();
        // // facades
        // // validate, send and persist
        // let (response, negotiation_process) = self
        //     .validate_and_send(
        //         &negotiation_process,
        //         Arc::new(negotiation_process_into_trait.clone()),
        //         peer_url_id,
        //         "offers",
        //     )
        //     .await?;
        // // bye!
        // let response =
        //     RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        // Ok(response)
        todo!()
    }

    async fn setup_negotiation_agreement_rpc(
        &self,
        input: &RpcNegotiationAgreementMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationAgreementMessageDto>> {
        // self.validator.negotiation_agreement_rpc(input).await?;
        // // get from input
        // let input_negotiation_id = input.get_consumer_pid().unwrap();
        // // fetch current process
        // let negotiation_process =
        //     self.persistence_service.fetch_process(input_negotiation_id.to_string().as_str()).await?;
        // let provider_pid = negotiation_process.identifiers.get("providerPid").unwrap();
        // let consumer_pid = negotiation_process.identifiers.get("consumerPid").unwrap();
        // // create message
        // let agreement = input.get_agreement().unwrap();
        // let negotiation_process_into_trait = NegotiationAgreementMessageDto {
        //     provider_pid: Urn::from_str(provider_pid.as_str())?,
        //     consumer_pid: Urn::from_str(consumer_pid.as_str())?,
        //     agreement,
        // };
        // // get uri
        // let identifier_key = match negotiation_process.inner.role.as_str() {
        //     "Provider" => "consumerPid",
        //     "Consumer" => "providerPid",
        //     _ => "providerPid",
        // };
        // let peer_url_id = negotiation_process.identifiers.get(identifier_key).unwrap();
        // // facades
        // // validate, send and persist
        // let (response, negotiation_process) = self
        //     .validate_and_send(
        //         &negotiation_process,
        //         Arc::new(negotiation_process_into_trait.clone()),
        //         peer_url_id,
        //         "agreement",
        //     )
        //     .await?;
        // // bye!
        // let response =
        //     RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        // Ok(response)
        todo!()
    }

    async fn setup_negotiation_agreement_verification_rpc(
        &self,
        input: &RpcNegotiationVerificationMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationVerificationMessageDto>> {
        // self.validator.negotiation_agreement_verification_rpc(input).await?;
        // // get from input
        // let input_negotiation_id = input.get_consumer_pid().unwrap();
        // // fetch current process
        // let negotiation_process =
        //     self.persistence_service.fetch_process(input_negotiation_id.to_string().as_str()).await?;
        // let provider_pid = negotiation_process.identifiers.get("providerPid").unwrap();
        // let consumer_pid = negotiation_process.identifiers.get("consumerPid").unwrap();
        // // create message
        // let agreement = input.get_agreement().unwrap();
        // let negotiation_process_into_trait = NegotiationVerificationMessageDto {
        //     provider_pid: Urn::from_str(provider_pid.as_str())?,
        //     consumer_pid: Urn::from_str(consumer_pid.as_str())?,
        // };
        // // get uri
        // let identifier_key = match negotiation_process.inner.role.as_str() {
        //     "Provider" => "consumerPid",
        //     "Consumer" => "providerPid",
        //     _ => "providerPid",
        // };
        // let peer_url_id = negotiation_process.identifiers.get(identifier_key).unwrap();
        // // facades
        // // validate, send and persist
        // let (response, negotiation_process) = self
        //     .validate_and_send(
        //         &negotiation_process,
        //         Arc::new(negotiation_process_into_trait.clone()),
        //         peer_url_id,
        //         "agreement/verification",
        //     )
        //     .await?;
        // // bye!
        // let response =
        //     RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        // Ok(response)
        todo!()
    }

    async fn setup_negotiation_event_accepted_rpc(
        &self,
        input: &RpcNegotiationEventAcceptedMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationEventAcceptedMessageDto>> {
        // self.validator.negotiation_event_accepted_rpc(input).await?;
        // // get from input
        // let input_negotiation_id = input.get_consumer_pid().unwrap();
        // // fetch current process
        // let negotiation_process =
        //     self.persistence_service.fetch_process(input_negotiation_id.to_string().as_str()).await?;
        // let provider_pid = negotiation_process.identifiers.get("providerPid").unwrap();
        // let consumer_pid = negotiation_process.identifiers.get("consumerPid").unwrap();
        // // create message
        // let agreement = input.get_agreement().unwrap();
        // let negotiation_process_into_trait = NegotiationEventMessageDto {
        //     provider_pid: Urn::from_str(provider_pid.as_str())?,
        //     consumer_pid: Urn::from_str(consumer_pid.as_str())?,
        //     event_type: NegotiationEventType::ACCEPTED,
        // };
        // // get uri
        // let identifier_key = match negotiation_process.inner.role.as_str() {
        //     "Provider" => "consumerPid",
        //     "Consumer" => "providerPid",
        //     _ => "providerPid",
        // };
        // let peer_url_id = negotiation_process.identifiers.get(identifier_key).unwrap();
        // // facades
        // // validate, send and persist
        // let (response, negotiation_process) = self
        //     .validate_and_send(
        //         &negotiation_process,
        //         Arc::new(negotiation_process_into_trait.clone()),
        //         peer_url_id,
        //         "events",
        //     )
        //     .await?;
        // // bye!
        // let response =
        //     RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        // Ok(response)
        todo!()
    }

    async fn setup_negotiation_event_finalized_rpc(
        &self,
        input: &RpcNegotiationEventFinalizedMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationEventFinalizedMessageDto>> {
        // self.validator.negotiation_event_finalized_rpc(input).await?;
        // // get from input
        // let input_negotiation_id = input.get_consumer_pid().unwrap();
        // // fetch current process
        // let negotiation_process =
        //     self.persistence_service.fetch_process(input_negotiation_id.to_string().as_str()).await?;
        // let provider_pid = negotiation_process.identifiers.get("providerPid").unwrap();
        // let consumer_pid = negotiation_process.identifiers.get("consumerPid").unwrap();
        // // create message
        // let agreement = input.get_agreement().unwrap();
        // let negotiation_process_into_trait = NegotiationEventMessageDto {
        //     provider_pid: Urn::from_str(provider_pid.as_str())?,
        //     consumer_pid: Urn::from_str(consumer_pid.as_str())?,
        //     event_type: NegotiationEventType::FINALIZED,
        // };
        // // get uri
        // let identifier_key = match negotiation_process.inner.role.as_str() {
        //     "Provider" => "consumerPid",
        //     "Consumer" => "providerPid",
        //     _ => "providerPid",
        // };
        // let peer_url_id = negotiation_process.identifiers.get(identifier_key).unwrap();
        // // facades
        // // validate, send and persist
        // let (response, negotiation_process) = self
        //     .validate_and_send(
        //         &negotiation_process,
        //         Arc::new(negotiation_process_into_trait.clone()),
        //         peer_url_id,
        //         "events",
        //     )
        //     .await?;
        // // bye!
        // let response =
        //     RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        // Ok(response)
        todo!()
    }

    async fn setup_negotiation_termination_rpc(
        &self,
        input: &RpcNegotiationTerminationMessageDto,
    ) -> anyhow::Result<RpcNegotiationMessageDto<RpcNegotiationTerminationMessageDto>> {
        // self.validator.negotiation_termination_rpc(input).await?;
        // // get from input
        // let input_negotiation_id = input.get_consumer_pid().unwrap();
        // // fetch current process
        // let negotiation_process =
        //     self.persistence_service.fetch_process(input_negotiation_id.to_string().as_str()).await?;
        // let provider_pid = negotiation_process.identifiers.get("providerPid").unwrap();
        // let consumer_pid = negotiation_process.identifiers.get("consumerPid").unwrap();
        // // create message
        // let code = input.get_error_code();
        // let reason = input.get_error_reason();
        // let negotiation_process_into_trait = NegotiationTerminationMessageDto {
        //     provider_pid: Urn::from_str(provider_pid.as_str())?,
        //     consumer_pid: Urn::from_str(consumer_pid.as_str())?,
        //     code,
        //     reason,
        // };
        // // get uri
        // let identifier_key = match negotiation_process.inner.role.as_str() {
        //     "Provider" => "consumerPid",
        //     "Consumer" => "providerPid",
        //     _ => "providerPid",
        // };
        // let peer_url_id = negotiation_process.identifiers.get(identifier_key).unwrap();
        // // facades
        // // validate, send and persist
        // let (response, negotiation_process) = self
        //     .validate_and_send(
        //         &negotiation_process,
        //         Arc::new(negotiation_process_into_trait.clone()),
        //         peer_url_id,
        //         "events",
        //     )
        //     .await?;
        // // bye!
        // let response =
        //     RpcNegotiationMessageDto { request: input.clone(), response, negotiation_agent_model: negotiation_process };
        // Ok(response)
        todo!()
    }
}

impl RPCOrchestratorService {
    // async fn validate_and_send<T>(
    //     &self,
    //     negotiation_process: &NegotiationProcessDto,
    //     payload: Arc<T>,
    //     peer_url_id: &str,
    //     url_suffix: &str,
    // ) -> anyhow::Result<(
    //     NegotiationProcessMessageWrapper<NegotiationAckMessageDto>,
    //     NegotiationProcessDto,
    // )>
    // where
    //     T: NegotiationProcessMessageTrait + 'static,
    // {
    //     // self.state_machine_service.validate_transition(None, payload.clone()).await?;
    //     // self.validator_service.validate(None, payload.clone()).await?;
    //     // where to send
    //     let callback_url = negotiation_process.inner.callback_address.clone().unwrap_or("".to_string());
    //     let peer_url = format!(
    //         "{}/negotiations/{}/{}",
    //         callback_url, peer_url_id, url_suffix
    //     );
    //     // create final message
    //     let message = NegotiationProcessMessageWrapper {
    //         context: ContextField::default(),
    //         _type: payload.get_message(),
    //         dto: payload.as_ref().clone(),
    //     };
    //     // send message to peer url
    //     let response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto> =
    //         self.http_client.post_json(peer_url.as_str(), &message).await?;
    //     // persist
    //     let negotiation_process = self
    //         .persistence_service
    //         .update(
    //             &negotiation_process.inner.id,
    //             &payload.as_ref().clone(),
    //         )
    //         .await?;
    //     // bye!
    //     Ok((response, negotiation_process))
    // }
}
