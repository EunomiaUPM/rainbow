#![allow(unused)]
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

mod errors;
pub(crate) mod facades;
pub(crate) mod http;
pub(crate) mod orchestrator;
mod persistence;
pub(crate) mod protocol_types;
pub(crate) mod validator;

use crate::entities::agreement::NegotiationAgentAgreementsTrait;
use crate::entities::negotiation_message::NegotiationAgentMessagesTrait;
use crate::entities::negotiation_process::NegotiationAgentProcessesTrait;
use crate::entities::offer::NegotiationAgentOffersTrait;
use crate::protocols::protocol::ProtocolPluginTrait;
use axum::Router;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::http_client::HttpClient;
use std::sync::Arc;

pub struct NegotiationDSP {
    negotiation_agent_process_entities: Arc<dyn NegotiationAgentProcessesTrait>,
    negotiation_agent_message_service: Arc<dyn NegotiationAgentMessagesTrait>,
    negotiation_offer_service: Arc<dyn NegotiationAgentOffersTrait>,
    negotiation_agreement_service: Arc<dyn NegotiationAgentAgreementsTrait>,
    config: Arc<ApplicationProviderConfig>,
}

impl NegotiationDSP {
    pub fn new(
        negotiation_agent_process_entities: Arc<dyn NegotiationAgentProcessesTrait>,
        negotiation_agent_message_service: Arc<dyn NegotiationAgentMessagesTrait>,
        negotiation_offer_service: Arc<dyn NegotiationAgentOffersTrait>,
        negotiation_agreement_service: Arc<dyn NegotiationAgentAgreementsTrait>,
        config: Arc<ApplicationProviderConfig>,
    ) -> Self {
        Self {
            negotiation_agent_message_service,
            negotiation_agent_process_entities,
            negotiation_offer_service,
            negotiation_agreement_service,
            config,
        }
    }
}

#[async_trait::async_trait]
impl ProtocolPluginTrait for NegotiationDSP {
    fn name(&self) -> &'static str {
        "Dataspace Protocol"
    }

    fn version(&self) -> &'static str {
        "1.0"
    }

    fn short_name(&self) -> &'static str {
        "DSP"
    }

    async fn build_router(&self) -> anyhow::Result<Router> {
        let http_client = Arc::new(HttpClient::new(10, 10));

        // Validator
        // let validator_helper = Arc::new(ValidationHelperService::new(
        //     self.transfer_agent_process_entities.clone(),
        // ));
        // let validator_payload = Arc::new(ValidatePayloadService::new(validator_helper.clone()));
        // let validator_state_machine_dsp = Arc::new(ValidatedStateTransitionServiceForDsp::new(
        //     validator_helper.clone(),
        // ));
        // let dsp_validator = Arc::new(ValidationDspStepsService::new(
        //     validator_payload.clone(),
        //     validator_state_machine_dsp.clone(),
        //     validator_helper.clone(),
        // ));
        // let validator_state_machine_rcp = Arc::new(ValidatedStateTransitionServiceForRcp::new(
        //     validator_helper.clone(),
        // ));
        // let rcp_validator = Arc::new(ValidationRpcStepsService::new(
        //     validator_payload.clone(),
        //     validator_state_machine_rcp.clone(),
        //     validator_helper.clone(),
        // ));

        // http service
        // let persistence_protocol_service = Arc::new(TransferPersistenceForProtocolService::new(
        //     self.transfer_agent_message_service.clone(),
        //     self.transfer_agent_process_entities.clone(),
        // ));
        // let persistence_rpc_service = Arc::new(TransferPersistenceForRpcService::new(
        //     self.transfer_agent_message_service.clone(),
        //     self.transfer_agent_process_entities.clone(),
        // ));

        // dataplane
        // let dataplane = DataplaneSetup::new();
        // let dataplane_controller = dataplane.get_data_plane_controller(self.config.clone()).await;
        // let data_plane_facade = Arc::new(DataPlaneProviderFacade::new(dataplane_controller.clone()));

        // data service resolver
        // let data_service_resolver = Arc::new(DataServiceFacadeServiceForDSProtocol::new(
        //     self.config.clone(),
        //     http_client.clone(),
        // ));

        // facades
        // let facades = Arc::new(FacadeService::new(
        //     data_service_resolver.clone(),
        //     data_plane_facade.clone(),
        // ));

        // orchestrators
        // let http_orchestator = Arc::new(ProtocolOrchestratorService::new(
        //     dsp_validator.clone(),
        //     persistence_protocol_service.clone(),
        //     facades.clone(),
        //     self.config.clone(),
        // ));
        // let rpc_orchestator = Arc::new(RPCOrchestratorService::new(
        //     rcp_validator.clone(),
        //     persistence_rpc_service,
        //     self.config.clone(),
        //     http_client.clone(),
        // ));
        // let orchestrator_service = Arc::new(OrchestratorService::new(
        //     http_orchestator.clone(),
        //     rpc_orchestator.clone(),
        // ));

        // router
        // let dsp_router = DspRouter::new(orchestrator_service.clone(), self.config.clone());
        // let rcp_router = RpcRouter::new(orchestrator_service.clone(), self.config.clone());

        // Ok(Router::new().merge(dsp_router.router()).merge(rcp_router.router()))
        Ok(Router::new())
    }

    fn build_grpc_router(&self) -> anyhow::Result<Option<Router>> {
        todo!()
    }
}
