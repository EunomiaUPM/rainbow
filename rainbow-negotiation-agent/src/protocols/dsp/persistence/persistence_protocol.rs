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
use crate::entities::agreement::{
    EditAgreementDto, NegotiationAgentAgreementsTrait, NewAgreementDto,
};
use crate::entities::negotiation_message::{
    NegotiationAgentMessagesTrait, NewNegotiationMessageDto,
};
use crate::entities::negotiation_process::{
    EditNegotiationProcessDto, NegotiationAgentProcessesTrait, NegotiationProcessDto,
    NewNegotiationProcessDto,
};
use crate::entities::offer::{NegotiationAgentOffersTrait, NewOfferDto};
use crate::protocols::dsp::persistence::NegotiationPersistenceTrait;
use crate::protocols::dsp::protocol_types::{
    NegotiationProcessMessageTrait, NegotiationProcessMessageType, NegotiationProcessState,
};
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::dsp_common::odrl::ContractRequestMessageOfferTypes;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct NegotiationPersistenceForProtocolService {
    pub negotiation_process_service: Arc<dyn NegotiationAgentProcessesTrait>,
    pub negotiation_messages_service: Arc<dyn NegotiationAgentMessagesTrait>,
    pub offer_service: Arc<dyn NegotiationAgentOffersTrait>,
    pub agreement_service: Arc<dyn NegotiationAgentAgreementsTrait>,
}

impl NegotiationPersistenceForProtocolService {
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
}

#[async_trait::async_trait]
impl NegotiationPersistenceTrait for NegotiationPersistenceForProtocolService {
    async fn get_negotiation_process_service(
        &self,
    ) -> anyhow::Result<Arc<dyn NegotiationAgentProcessesTrait>> {
        Ok(self.negotiation_process_service.clone())
    }

    async fn get_negotiation_message_service(
        &self,
    ) -> anyhow::Result<Arc<dyn NegotiationAgentMessagesTrait>> {
        Ok(self.negotiation_messages_service.clone())
    }

    async fn get_negotiation_offer_service(
        &self,
    ) -> anyhow::Result<Arc<dyn NegotiationAgentOffersTrait>> {
        Ok(self.offer_service.clone())
    }

    async fn get_negotiation_agreement_service(
        &self,
    ) -> anyhow::Result<Arc<dyn NegotiationAgentAgreementsTrait>> {
        Ok(self.agreement_service.clone())
    }

    async fn fetch_process(&self, id: &str) -> anyhow::Result<NegotiationProcessDto> {
        let id_urn = Urn::from_str(id)?;
        let process = self
            .get_negotiation_process_service()
            .await?
            .get_negotiation_process_by_key_value(&id_urn)
            .await?
            .ok_or_else(|| {
                let err = CommonErrors::missing_resource_new(
                    id_urn.to_string().as_str(),
                    "Process service not found",
                );
                error!("{}", err.log());
                err
            })?;
        Ok(process)
    }

    async fn create_process(
        &self,
        protocol: &str,
        direction: &str,
        peer_address: Option<String>,
        provider_address: Option<String>,
        ack_message_dto: Arc<dyn NegotiationProcessMessageTrait>,
        payload_dto: Arc<dyn NegotiationProcessMessageTrait>,
        payload_value: Value,
    ) -> anyhow::Result<NegotiationProcessDto> {
        // general types
        let dto_message_type = payload_dto.get_message();
        let role_from_message_type = match dto_message_type {
            NegotiationProcessMessageType::NegotiationRequestMessage => RoleConfig::Provider,
            NegotiationProcessMessageType::NegotiationOfferMessage => RoleConfig::Consumer,
            _ => RoleConfig::Provider,
        };
        let state_from_message_type = match dto_message_type {
            NegotiationProcessMessageType::NegotiationRequestMessage => {
                NegotiationProcessState::Requested
            }
            NegotiationProcessMessageType::NegotiationOfferMessage => {
                NegotiationProcessState::Offered
            }
            _ => NegotiationProcessState::Requested,
        };
        let negotiation_process_id =
            Urn::from_str(format!("urn:negotiation-process:{}", uuid::Uuid::new_v4()).as_str())?;

        // dsp identifiers
        let mut identifiers = HashMap::new();
        let provider_pid = match dto_message_type {
            NegotiationProcessMessageType::NegotiationRequestMessage => {
                format!("urn:provider-pid:{}", uuid::Uuid::new_v4())
            }
            NegotiationProcessMessageType::NegotiationOfferMessage => {
                payload_dto.get_provider_pid().unwrap().to_string()
            }
            _ => "".to_string(),
        };
        identifiers.insert("providerPid".to_string(), provider_pid);
        let consumer_pid = match dto_message_type {
            NegotiationProcessMessageType::NegotiationRequestMessage => {
                payload_dto.get_consumer_pid().unwrap().to_string()
            }
            NegotiationProcessMessageType::NegotiationOfferMessage => {
                format!("urn:consumer-pid:{}", uuid::Uuid::new_v4())
            }
            _ => "".to_string(),
        };
        identifiers.insert("consumerPid".to_string(), consumer_pid);

        // process
        let mut negotiation_process = self
            .get_negotiation_process_service()
            .await?
            .create_negotiation_process(&NewNegotiationProcessDto {
                id: Some(negotiation_process_id.clone()),
                state: state_from_message_type.to_string(),
                state_attribute: None,
                associated_agent_peer: "".to_string(),
                protocol: protocol.to_string(),
                callback_address: payload_dto.get_callback_address().clone(),
                role: role_from_message_type.to_string(),
                properties: None,
                identifiers: Some(identifiers),
            })
            .await?;

        // message
        let message_type = payload_dto.get_message();
        let negotiation_message_id =
            Urn::from_str(format!("urn:negotiation-message:{}", uuid::Uuid::new_v4()).as_str())?;
        let negotiation_message = self
            .get_negotiation_message_service()
            .await?
            .create_negotiation_message(&NewNegotiationMessageDto {
                id: Some(negotiation_message_id.clone()),
                negotiation_agent_process_id: negotiation_process_id.clone(),
                direction: direction.to_string(),
                protocol: protocol.to_string(),
                message_type: message_type.to_string(),
                state_transition_from: "-".to_string(),
                state_transition_to: state_from_message_type.to_string(),
                payload: payload_value,
            })
            .await?;
        negotiation_process.messages.push(negotiation_message.inner);

        // offer
        let offer_id = match payload_dto.get_offer().unwrap() {
            ContractRequestMessageOfferTypes::OfferMessage(m) => m.id,
            ContractRequestMessageOfferTypes::OfferId(i) => i.id,
        };
        let offer = payload_dto.get_offer().unwrap();
        let negotiation_offer = self
            .get_negotiation_offer_service()
            .await?
            .create_offer(&NewOfferDto {
                id: None,
                negotiation_agent_process_id: negotiation_process_id.clone(),
                negotiation_agent_message_id: negotiation_message_id.clone(),
                offer_id: offer_id.to_string(),
                offer_content: serde_json::to_value(offer).unwrap(),
            })
            .await?;
        negotiation_process.offers.push(negotiation_offer.inner);
        Ok(negotiation_process)
    }

    async fn update_process(
        &self,
        id: &str,
        payload_dto: Arc<dyn NegotiationProcessMessageTrait>,
        payload_value: Value,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let urn_id = Urn::from_str(id).expect("Failed to parse urnID");
        let message_type = payload_dto.get_message();
        // new state request
        let new_state: NegotiationProcessState =
            NegotiationProcessState::from(message_type.clone());
        // current state
        let negotiation_process = self
            .get_negotiation_process_service()
            .await?
            .get_negotiation_process_by_key_value(&urn_id)
            .await?
            .ok_or_else(|| {
                let err = CommonErrors::missing_resource_new(
                    urn_id.to_string().as_str(),
                    "Process service not found",
                );
                error!("{}", err.log());
                err
            })?;
        let negotiation_process_urn = Urn::from_str(negotiation_process.inner.id.as_str())?;
        // update
        let negotiation_process_urn = Urn::from_str(negotiation_process.inner.id.as_str())?;
        // role
        let role = negotiation_process.inner.role.parse::<RoleConfig>()?;

        // transfer_process
        let mut negotiation_process = self
            .get_negotiation_process_service()
            .await?
            .put_negotiation_process(
                &negotiation_process_urn,
                &EditNegotiationProcessDto {
                    state: Some(new_state.to_string()),
                    state_attribute: None,
                    properties: None,
                    error_details: None,
                    identifiers: None,
                },
            )
            .await?;

        // message
        let message_type = payload_dto.get_message();
        let negotiation_message_id =
            Urn::from_str(format!("urn:negotiation-message:{}", uuid::Uuid::new_v4()).as_str())?;
        let negotiation_message = self
            .get_negotiation_message_service()
            .await?
            .create_negotiation_message(&NewNegotiationMessageDto {
                id: Some(negotiation_message_id.clone()),
                negotiation_agent_process_id: negotiation_process_urn.clone(),
                direction: "INBOUND".to_string(),
                protocol: "DSP".to_string(),
                message_type: message_type.to_string(),
                state_transition_from: negotiation_process.inner.state.to_string(),
                state_transition_to: new_state.to_string(),
                payload: payload_value,
            })
            .await?;
        negotiation_process.messages.push(negotiation_message.inner);

        // create offer or agreement
        match message_type {
            NegotiationProcessMessageType::NegotiationRequestMessage
            | NegotiationProcessMessageType::NegotiationOfferMessage => {
                // offer
                let offer_id = match payload_dto.get_offer().unwrap() {
                    ContractRequestMessageOfferTypes::OfferMessage(m) => m.id,
                    ContractRequestMessageOfferTypes::OfferId(i) => i.id,
                };
                let offer = payload_dto.get_offer().unwrap();
                let negotiation_offer = self
                    .get_negotiation_offer_service()
                    .await?
                    .create_offer(&NewOfferDto {
                        id: None,
                        negotiation_agent_process_id: negotiation_process_urn.clone(),
                        negotiation_agent_message_id: negotiation_message_id.clone(),
                        offer_id: offer_id.to_string(),
                        offer_content: serde_json::to_value(offer).unwrap(),
                    })
                    .await?;
                negotiation_process.offers.push(negotiation_offer.inner);
            }
            NegotiationProcessMessageType::NegotiationAgreementMessage => {
                // agreement
                let agreement = payload_dto.get_agreement().unwrap();
                let target = agreement.clone().target;
                let negotiation_agreement = self
                    .get_negotiation_agreement_service()
                    .await?
                    .create_agreement(&NewAgreementDto {
                        id: None,
                        negotiation_agent_process_id: negotiation_process_urn.clone(),
                        negotiation_agent_message_id: negotiation_message_id.clone(),
                        // TODO consumer_participant_id && provider_participant_id
                        consumer_participant_id: "".to_string(),
                        provider_participant_id: "".to_string(),
                        agreement_content: serde_json::to_value(agreement).unwrap(),
                        target,
                    })
                    .await?;
                negotiation_process.agreement = Some(negotiation_agreement.inner);
            }
            NegotiationProcessMessageType::NegotiationAgreementVerificationMessage => {
                // active agreement
                let agreement_id = self
                    .get_negotiation_agreement_service()
                    .await?
                    .get_agreement_by_negotiation_process(&negotiation_process_urn)
                    .await?
                    .unwrap();
                let agreement_id = Urn::from_str(agreement_id.inner.id.as_str())?;
                let negotiation_agreement = self
                    .get_negotiation_agreement_service()
                    .await?
                    .put_agreement(
                        &agreement_id,
                        &EditAgreementDto { state: Some("ACTIVE".to_string()) },
                    )
                    .await?;
                negotiation_process.agreement = Some(negotiation_agreement.inner);
            }
            _ => {}
        }

        Ok(negotiation_process)
    }
}
