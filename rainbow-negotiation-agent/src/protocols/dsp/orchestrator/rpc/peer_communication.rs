use crate::entities::negotiation_process::NegotiationProcessDto;
use crate::protocols::dsp::orchestrator::rpc::types::RpcNegotiationProcessMessageTrait;
use crate::protocols::dsp::orchestrator::traits::orchestration_helpers::OrchestrationHelpers;
use crate::protocols::dsp::protocol_types::{NegotiationProcessMessageTrait, NegotiationProcessMessageType};
use anyhow::anyhow;
use rainbow_common::http_client::HttpClient;
use std::sync::Arc;

pub struct PeerCommunication {
    http_client: Arc<HttpClient>,
}

impl PeerCommunication {
    pub fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }
    pub async fn send_from_rpc(&self, payload: &dyn RpcNegotiationProcessMessageTrait) -> anyhow::Result<()> {
        let peer_address = self.get_rpc_message_safely(payload)?;
        // let suffix = match peer_address {
        //     NegotiationProcessMessageType::NegotiationRequestMessage => {}
        //     NegotiationProcessMessageType::NegotiationOfferMessage => {}
        //     NegotiationProcessMessageType::NegotiationEventMessage(_) => {}
        //     NegotiationProcessMessageType::NegotiationAgreementMessage => {}
        //     NegotiationProcessMessageType::NegotiationAgreementVerificationMessage => {}
        //     NegotiationProcessMessageType::NegotiationTerminationMessage => {}
        //     _ => {
        //         return Err(anyhow!("this message is not for rpc sending"))
        //     }
        // }
        Ok(())
    }
    pub async fn send_with_dto(&self, dto: &NegotiationProcessDto, payload: &dyn NegotiationProcessMessageTrait) {
        todo!()
    }
}

impl OrchestrationHelpers for PeerCommunication {}
