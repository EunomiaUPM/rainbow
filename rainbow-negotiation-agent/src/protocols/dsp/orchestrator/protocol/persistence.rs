use crate::entities::agreement::{
    AgreementDto, EditAgreementDto, NegotiationAgentAgreementsTrait, NewAgreementDto,
};
use crate::entities::negotiation_message::{
    NegotiationAgentMessagesTrait, NegotiationMessageDto, NewNegotiationMessageDto,
};
use crate::entities::negotiation_process::{
    EditNegotiationProcessDto, NegotiationAgentProcessesTrait, NegotiationProcessDto,
    NewNegotiationProcessDto,
};
use crate::entities::offer::{NegotiationAgentOffersTrait, NewOfferDto, OfferDto};
use crate::protocols::dsp::orchestrator::rpc::types::RpcNegotiationProcessMessageTrait;
use crate::protocols::dsp::orchestrator::traits::orchestration_extractors::OrchestrationExtractors;
use crate::protocols::dsp::orchestrator::traits::orchestration_helpers::OrchestrationHelpers;
use crate::protocols::dsp::protocol_types::{
    NegotiationProcessMessageTrait, NegotiationProcessMessageType, NegotiationProcessState,
};
use anyhow::bail;
use async_trait::async_trait;
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::dsp_common::odrl::ContractRequestMessageOfferTypes;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct OrchestrationPersistenceForProtocol {
    negotiation_process_service: Arc<dyn NegotiationAgentProcessesTrait>,
    negotiation_messages_service: Arc<dyn NegotiationAgentMessagesTrait>,
    offer_service: Arc<dyn NegotiationAgentOffersTrait>,
    agreement_service: Arc<dyn NegotiationAgentAgreementsTrait>,
}

impl OrchestrationPersistenceForProtocol {
    pub fn new(
        negotiation_process_service: Arc<dyn NegotiationAgentProcessesTrait>,
        negotiation_messages_service: Arc<dyn NegotiationAgentMessagesTrait>,
        offer_service: Arc<dyn NegotiationAgentOffersTrait>,
        agreement_service: Arc<dyn NegotiationAgentAgreementsTrait>,
    ) -> Self {
        Self {
            negotiation_process_service,
            negotiation_messages_service,
            offer_service,
            agreement_service,
        }
    }

    pub async fn create_new(
        &self,
        payload: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let mut process = self.create_process(payload).await?;
        let process_id = self.convert_string_to_urn(&process.inner.id)?;
        let message = self.create_message(&process_id, payload, &process).await?;
        let message_id = self.convert_string_to_urn(&message.inner.id)?;
        let offer = self.create_offer(&process_id, &message_id, payload).await?;
        process.messages.push(message.inner);
        process.offers.push(offer.inner);
        Ok(process)
    }

    pub async fn update(
        &self,
        identifier: &str,
        payload: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let process = self.fetch_process(identifier).await?;
        let process_id = self.convert_string_to_urn(&process.inner.id)?;
        let mut new_process = self.update_process(&process_id, payload).await?;
        let message = self.create_message(&process_id, payload, &process).await?;
        new_process.messages.push(message.inner);
        Ok(new_process)
    }

    pub async fn update_with_offer(
        &self,
        identifier: &str,
        payload: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let process = self.fetch_process(identifier).await?;
        let process_id = self.convert_string_to_urn(&process.inner.id)?;
        let mut new_process = self.update_process(&process_id, payload).await?;
        let message = self.create_message(&process_id, payload, &process).await?;
        let message_id = self.convert_string_to_urn(&message.inner.id)?;
        let offer = self.create_offer(&process_id, &message_id, payload).await?;
        new_process.messages.push(message.inner);
        new_process.offers.push(offer.inner);
        Ok(new_process)
    }

    pub async fn update_with_new_agreement(
        &self,
        identifier: &str,
        payload: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let process = self.fetch_process(identifier).await?;
        let associated_agent_peer = process.inner.associated_agent_peer.clone();
        let process_id = self.convert_string_to_urn(&process.inner.id)?;
        let mut new_process = self.update_process(&process_id, payload).await?;
        let message = self.create_message(&process_id, payload, &process).await?;
        let message_id = self.convert_string_to_urn(&message.inner.id)?;
        let agreement = self
            .create_agreement(&process_id, &message_id, &associated_agent_peer, payload)
            .await?;
        new_process.messages.push(message.inner);
        new_process.agreement = Some(agreement.inner);
        Ok(new_process)
    }

    pub async fn update_with_agreement(
        &self,
        identifier: &str,
        payload: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let process = self.fetch_process(identifier).await?;
        let process_id = self.convert_string_to_urn(&process.inner.id)?;
        let mut new_process = self.update_process(&process_id, payload).await?;
        let message = self.create_message(&process_id, payload, &process).await?;
        let message_id = self.convert_string_to_urn(&message.inner.id)?;
        let agreement = self.update_agreement(&process_id, &message_id, payload).await?;
        new_process.messages.push(message.inner);
        new_process.agreement = Some(agreement.inner);
        Ok(new_process)
    }
}

impl OrchestrationHelpers for OrchestrationPersistenceForProtocol {}
impl OrchestrationExtractors for OrchestrationPersistenceForProtocol {
    fn get_role_from_message_type(
        &self,
        message: &NegotiationProcessMessageType,
    ) -> anyhow::Result<RoleConfig> {
        match message {
            NegotiationProcessMessageType::NegotiationRequestMessage => Ok(RoleConfig::Provider),
            NegotiationProcessMessageType::NegotiationOfferMessage => Ok(RoleConfig::Consumer),
            _ => {
                let err = CommonErrors::parse_new("Message not allowed here");
                error!("{}", err.log());
                bail!(err);
            }
        }
    }
}

impl OrchestrationPersistenceForProtocol {
    pub async fn fetch_process(&self, id: &str) -> anyhow::Result<NegotiationProcessDto> {
        let urn = self.convert_str_to_urn(id)?;
        let process = self
            .negotiation_process_service
            .get_negotiation_process_by_key_value(&urn)
            .await?
            .ok_or_else(|| {
                CommonErrors::missing_resource_new(urn.to_string().as_str(), "Process not found")
            })?;
        Ok(process)
    }

    async fn create_process(
        &self,
        message: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let id = self.create_entity_urn("negotiation-process")?;
        let message_type = self.get_dsp_message_safely(message)?;
        let state: NegotiationProcessState = message_type.clone().into();
        let callback = self.get_dsp_callback_address_safely(message)?;
        let role = self.get_role_from_message_type(&message_type)?;
        let key_identifier = match role {
            RoleConfig::Provider => "consumerPid",
            RoleConfig::Consumer => "providerPid",
            _ => "id",
        };
        let not_key_identifier = match role {
            RoleConfig::Provider => "providerPid",
            RoleConfig::Consumer => "consumerPid",
            _ => "id",
        };
        let not_key_identifier_id = match role {
            RoleConfig::Provider => "provider-pid",
            RoleConfig::Consumer => "consumer-pid",
            _ => "id",
        };
        let identifier = match role {
            RoleConfig::Provider => self.get_dsp_consumer_pid_safely(message)?,
            RoleConfig::Consumer => self.get_dsp_provider_pid_safely(message)?,
            _ => {
                let err = CommonErrors::parse_new(
                    "Something is wrong. Seems this process' state is not protocol compliant",
                );
                log::error!("{}", err.log());
                bail!(err);
            }
        };
        let mut identifiers = HashMap::new();
        identifiers.insert(key_identifier.to_string(), identifier.to_string());
        identifiers.insert(
            not_key_identifier.to_string(),
            self.create_entity_urn(not_key_identifier_id)?.to_string(),
        );

        let new_process = self
            .negotiation_process_service
            .create_negotiation_process(&NewNegotiationProcessDto {
                id: Some(id),
                state: state.to_string(),
                state_attribute: None, // O el valor por defecto que corresponda
                associated_agent_peer: "".to_string(), // O extraer del mensaje si existe
                protocol: "DSP".to_string(),
                callback_address: Some(callback),
                role: role.to_string(),
                properties: None,
                identifiers: Some(identifiers),
            })
            .await?;

        Ok(new_process)
    }

    async fn create_message(
        &self,
        process_id: &Urn,
        message: &dyn NegotiationProcessMessageTrait,
        process: &NegotiationProcessDto,
    ) -> anyhow::Result<NegotiationMessageDto> {
        let id = self.create_entity_urn("negotiation-message")?;
        let message_type = self.get_dsp_message_safely(message)?;
        let old_state = process.inner.state.parse::<NegotiationProcessState>().unwrap();
        let state: NegotiationProcessState = message_type.clone().into();
        let payload_json = message.as_json();

        let new_message = self
            .negotiation_messages_service
            .create_negotiation_message(&NewNegotiationMessageDto {
                id: Some(id),
                negotiation_agent_process_id: process_id.clone(),
                direction: "INBOUND".to_string(),
                protocol: "DSP".to_string(),
                message_type: message_type.to_string(),
                state_transition_from: old_state.to_string(),
                state_transition_to: state.to_string(),
                payload: payload_json,
            })
            .await?;
        Ok(new_message)
    }

    async fn create_offer(
        &self,
        process_id: &Urn,
        message_id: &Urn,
        message: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<OfferDto> {
        let id = self.create_entity_urn("offer")?;
        let offer_content = self.get_dsp_offer_safely(message)?;

        let offer_id = match &offer_content {
            ContractRequestMessageOfferTypes::OfferMessage(m) => &m.id,
            ContractRequestMessageOfferTypes::OfferId(i) => &i.id,
        }
        .to_string();

        let new_offer = self
            .offer_service
            .create_offer(&NewOfferDto {
                id: Some(id),
                negotiation_agent_process_id: process_id.clone(),
                negotiation_agent_message_id: message_id.clone(),
                offer_id,
                offer_content: serde_json::to_value(offer_content)?,
            })
            .await?;
        Ok(new_offer)
    }

    async fn create_agreement(
        &self,
        pid: &Urn,
        mid: &Urn,
        peer: &String,
        message: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<AgreementDto> {
        let id = self.create_entity_urn("agreement")?;
        let agreement = self.get_dsp_agreement_safely(message)?;
        let target = agreement.clone().target;
        let agr = self
            .agreement_service
            .create_agreement(&NewAgreementDto {
                id: Some(id),
                negotiation_agent_process_id: pid.clone(),
                negotiation_agent_message_id: mid.clone(),
                consumer_participant_id: peer.to_string(),
                provider_participant_id: "".to_string(),
                agreement_content: serde_json::to_value(agreement).unwrap(),
                target,
            })
            .await?;
        Ok(agr)
    }

    async fn update_agreement(
        &self,
        pid: &Urn,
        _mid: &Urn,
        message: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<AgreementDto> {
        let fetching_agreement = self
            .agreement_service
            .get_agreement_by_negotiation_process(pid)
            .await?
            .ok_or_else(|| {
                CommonErrors::missing_resource_new(pid.to_string().as_str(), "Agreement not found")
            })?;
        let agreement_urn = self.convert_string_to_urn(&fetching_agreement.inner.id)?;
        let agreement = self
            .agreement_service
            .put_agreement(
                &agreement_urn,
                &EditAgreementDto { state: Some("ACTIVE".to_string()) },
            )
            .await?;
        Ok(agreement)
    }

    async fn update_process(
        &self,
        pid: &Urn,
        message: &dyn NegotiationProcessMessageTrait,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let message_type = self.get_dsp_message_safely(message)?;
        let state: NegotiationProcessState = message_type.clone().into();
        let process = self
            .negotiation_process_service
            .put_negotiation_process(
                pid,
                &EditNegotiationProcessDto {
                    state: Some(state.to_string()),
                    state_attribute: None,
                    properties: None,
                    error_details: None,
                    identifiers: None,
                },
            )
            .await?;
        Ok(process)
    }
}
