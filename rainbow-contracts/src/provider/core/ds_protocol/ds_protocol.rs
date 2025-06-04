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

use crate::common::schemas::validation::validate_payload_schema;
use crate::common::schemas::{
    CONTRACT_AGREEMENT_MESSAGE_SCHEMA, CONTRACT_AGREEMENT_VERIFICATION_MESSAGE_SCHEMA,
    CONTRACT_NEGOTIATION_EVENT_MESSAGE_SCHEMA, CONTRACT_OFFER_MESSAGE_SCHEMA, CONTRACT_REQUEST_MESSAGE_SCHEMA,
    CONTRACT_TERMINATION_MESSAGE_SCHEMA,
};
use crate::provider::core::catalog_odrl_facade::CatalogOdrlFacadeTrait;
use crate::provider::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use crate::provider::core::ds_protocol::DSProtocolContractNegotiationProviderTrait;
use anyhow::bail;
use axum::async_trait;
use jsonschema::BasicOutput;
use log::error;
use rainbow_common::config::ConfigRoles;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{
    ContractNegotiationEventMessage, NegotiationEventType,
};
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_odrl::ContractRequestMessageOfferTypes;
use rainbow_common::protocol::contract::contract_protocol_trait::DSProtocolContractNegotiationMessageTrait;
use rainbow_common::protocol::contract::{ContractNegotiationMessages, ContractNegotiationState};
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_provider::entities::cn_process;
use rainbow_db::contracts_provider::repo::{
    AgreementRepo, ContractNegotiationMessageRepo, ContractNegotiationOfferRepo, ContractNegotiationProcessRepo,
    EditContractNegotiationProcess, NewContractNegotiationMessage, NewContractNegotiationOffer,
    NewContractNegotiationProcess, Participant,
};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value, Value};
use std::sync::Arc;
use tracing::debug;
use urn::Urn;

pub struct DSProtocolContractNegotiationProviderService<T, U, V>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Participant
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    V: CatalogOdrlFacadeTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
    catalog_facade: Arc<V>,
}

impl<T, U, V> DSProtocolContractNegotiationProviderService<T, U, V>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Participant
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    V: CatalogOdrlFacadeTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>, catalog_facade: Arc<V>) -> Self {
        Self { repo, notification_service, catalog_facade }
    }

    ///
    ///
    fn json_schema_validation<'a, M: DSProtocolContractNegotiationMessageTrait<'a>>(
        &self,
        message: &M,
    ) -> anyhow::Result<()> {
        debug!("Contract negotiation json_schema_validation");
        validate_payload_schema(message)?;
        Ok(())
    }

    ///
    ///
    async fn payload_validation<'a, M: DSProtocolContractNegotiationMessageTrait<'a>>(
        &self,
        incoming_provider_pid: Option<&Urn>,
        message: &M,
    ) -> anyhow::Result<Option<cn_process::Model>> {
        debug!("Contract negotiation payload_validation");

        // 1. provider in url and provider in body must coincide
        // In many bindings provider_pid is in body and url, these must coincide
        let provider_pid = message.get_provider_pid()?;
        match (incoming_provider_pid, provider_pid) {
            (None, _) => {}
            (Some(i), Some(p)) if i == p => {}
            out => bail!(IdsaCNError::ValidationError(format!(
                "Provider PID in path and in request body do not match. Path: {}, Request: {}",
                out.0.map(|u| u.to_string()).unwrap_or("".to_string()),
                out.1.map(|u| u.to_string()).unwrap_or("".to_string())
            ))),
        };

        // 2. there must be process correlation between provider pid and consumer pid
        // Ack and Error don't need this validation
        // Request only need this validation in case provider is some (re-request from consumer)
        match message.get_message_type()? {
            ContractNegotiationMessages::ContractNegotiationAck => Ok(None),
            ContractNegotiationMessages::ContractNegotiationError => Ok(None),
            ContractNegotiationMessages::ContractRequestMessage if incoming_provider_pid.is_none() => Ok(None),
            _ => {
                let cn_process_consumer = self
                    .repo
                    .get_cn_processes_by_consumer_id(message.get_consumer_pid()?.unwrap().to_owned())
                    .await
                    .map_err(IdsaCNError::DbErr)?
                    .ok_or(IdsaCNError::ProcessNotFound {
                        provider_pid: provider_pid.map(|p| p.to_owned()),
                        consumer_pid: Option::from(message.get_consumer_pid()?.map(|m| m.to_owned())),
                    })?;

                let cn_process_provider = self
                    .repo
                    .get_cn_processes_by_provider_id(&provider_pid.unwrap())
                    .await
                    .map_err(IdsaCNError::DbErr)?
                    .ok_or(IdsaCNError::ProcessNotFound {
                        provider_pid: provider_pid.map(|p| p.to_owned()),
                        consumer_pid: Option::from(message.get_consumer_pid()?.map(|m| m.to_owned())),
                    })?;
                if cn_process_consumer.cn_process_id != cn_process_provider.cn_process_id {
                    bail!(IdsaCNError::ValidationError(
                        "ConsumerPid and ProviderPid don't coincide".to_string()
                    ))
                }
                Ok(Option::from(cn_process_provider))
            }
        }
    }

    ///
    ///
    async fn transition_validation<'a, M: DSProtocolContractNegotiationMessageTrait<'a>>(
        &self,
        message: &M,
    ) -> anyhow::Result<()> {
        debug!("Contract negotiation transition_validation");
        // Negotiation state
        let consumer_pid = message.get_consumer_pid()?.to_owned();
        let provider_pid = message.get_provider_pid()?;
        let message_type = message.get_message_type()?;

        // 1. Only provider is optional in ContractRequestMessage
        match (provider_pid, message_type) {
            (None, m) => match m {
                ContractNegotiationMessages::ContractRequestMessage => {}
                _ => bail!("ProviderPid must be provided in this message"),
            },
            _ => {}
        }

        // extract process
        let cn_process = self
            .repo
            .get_cn_processes_by_consumer_id(consumer_pid.clone().unwrap().to_owned())
            .await
            .map_err(|e| IdsaCNError::DbErr(e.into()))?;

        // 2. A missing providerPid with cnProcess not allowed
        // Or a providerPid
        match (provider_pid, &cn_process) {
            // (Some(p), None) => {}
            // (None, None) => {}
            (None, Some(_)) => bail!(
                "Contract with consumerPid {} already requested",
                &consumer_pid.unwrap()
            ),
            _ => {}
        }

        // 3. transition matrix
        match message.get_message_type()? {
            ContractNegotiationMessages::ContractRequestMessage => match (&provider_pid, &cn_process) {
                (None, None) => {}
                (Some(_), Some(cn_process)) if cn_process.state == ContractNegotiationState::Offered.to_string() => {}
                _ => bail!("Message ContractRequestMessage not allowed"),
            },
            ContractNegotiationMessages::ContractOfferMessage => {
                bail!("Message ContractOfferMessage not allowed")
            }
            ContractNegotiationMessages::ContractAgreementMessage => {
                bail!("Message ContractAgreementMessage not allowed")
            }
            ContractNegotiationMessages::ContractAgreementVerificationMessage => match (&provider_pid, &cn_process) {
                (Some(_), Some(cn_process)) => {
                    if cn_process.state != ContractNegotiationState::Agreed.to_string() {
                        bail!("Message ContractAgreementVerificationMessage not allowed")
                    }
                }
                _ => bail!("Message ContractAgreementVerificationMessage not allowed"),
            },
            ContractNegotiationMessages::ContractNegotiationEventMessage => match (&provider_pid, &cn_process) {
                (Some(_), Some(cn_process)) => {
                    if cn_process.state != ContractNegotiationState::Offered.to_string() {
                        bail!("Message ContractNegotiationEventMessage not allowed")
                    }
                    match message.get_negotiation_event_type()? {
                        Some(ev) if ev == NegotiationEventType::Accepted => {}
                        _ => bail!("Event type must be ACCEPTED for ContractAgreementMessage"),
                    }
                }
                _ => bail!("Message ContractNegotiationEventMessage not allowed"),
            },
            ContractNegotiationMessages::ContractNegotiationTerminationMessage => match (&provider_pid, &cn_process) {
                (Some(_), Some(cn_process)) => match cn_process.state.parse::<ContractNegotiationState>()? {
                    ContractNegotiationState::Requested => {}
                    ContractNegotiationState::Offered => {}
                    ContractNegotiationState::Agreed => {}
                    _ => bail!("Message ContractNegotiationTerminationMessage not allowed"),
                },
                _ => bail!("Message ContractNegotiationTerminationMessage not allowed"),
            },
            m => bail!("Message {} not allowed", m.to_string()),
        }

        Ok(())
    }

    ///
    ///
    async fn notify_subscribers(&self, subcategory: String, message: Value) -> anyhow::Result<()> {
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
                subcategory,
                message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
                message_content: message,
                message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
            })
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<T, U, V> DSProtocolContractNegotiationProviderTrait for DSProtocolContractNegotiationProviderService<T, U, V>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Participant
    + Send
    + Sync
    + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    V: CatalogOdrlFacadeTrait + Send + Sync,
{
    async fn get_negotiation(&self, provider_pid: Urn) -> anyhow::Result<ContractAckMessage> {
        let cn_process = self
            .repo
            .get_cn_processes_by_provider_id(&provider_pid)
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound { provider_pid: Some(provider_pid), consumer_pid: None })?;
        Ok(cn_process.into())
    }

    async fn post_request(&self, input: ContractRequestMessage) -> anyhow::Result<ContractAckMessage> {
        // 1. validate request
        self.transition_validation(&input).await.map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let _ = self.payload_validation(None, &input).await.map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;

        // 2. resolve odrl policy
        let odrl_ids = match &input.odrl_offer {
            ContractRequestMessageOfferTypes::OfferId(offer) => Ok::<_, anyhow::Error>(offer.id.clone()),
            ContractRequestMessageOfferTypes::OfferMessage(offer) => Ok::<_, anyhow::Error>(offer.id.clone()),
        }?;
        let offer = match self.catalog_facade.resolve_odrl_offers(odrl_ids).await {
            Ok(resolver) => resolver,
            Err(_) => bail!(IdsaCNError::NotCheckedError {
                provider_pid: None,
                consumer_pid: None,
                error: "Id not found".to_string()
            }),
        };
        match &input.odrl_offer {
            ContractRequestMessageOfferTypes::OfferMessage(input_offer) => {
                if input_offer.target.clone() != offer.target.clone().unwrap() {
                    bail!(IdsaCNError::NotCheckedError {
                        provider_pid: None,
                        consumer_pid: None,
                        error: "target not coincide".to_string()
                    })
                }
            }
            ContractRequestMessageOfferTypes::OfferId(_) => {}
        }

        // 3. persist process, message and offer
        let cn_process = self
            .repo
            .create_cn_process(NewContractNegotiationProcess {
                provider_id: None,
                consumer_id: Option::from(input.consumer_pid.clone()),
                state: ContractNegotiationState::Requested,
                initiated_by: ConfigRoles::Consumer,
            })
            .await
            .map_err(IdsaCNError::DbErr)?;

        let cn_message = self
            .repo
            .create_cn_message(
                get_urn_from_string(&cn_process.cn_process_id)?,
                NewContractNegotiationMessage {
                    _type: input._type.to_string(),
                    from: "Consumer".to_string(),
                    to: "Provider".to_string(),
                    content: serde_json::to_value(&input).unwrap(),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        let cn_offer = self
            .repo
            .create_cn_offer(
                get_urn_from_string(&cn_process.cn_process_id)?,
                get_urn_from_string(&cn_message.cn_message_id)?,
                NewContractNegotiationOffer { offer_id: None, offer_content: serde_json::to_value(&offer)? },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        // 4. notify subscriptions
        self.notify_subscribers(
            "ContractRequestMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
                "offer": cn_offer
            }),
        )
            .await?;

        Ok(cn_process.into())
    }

    async fn post_provider_request(
        &self,
        provider_pid: Urn,
        input: ContractRequestMessage,
    ) -> anyhow::Result<ContractAckMessage> {
        // 1. validate request
        self.transition_validation(&input).await.map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let cn_process = self
            .payload_validation(Some(&provider_pid), &input)
            .await
            .map_err(|e| IdsaCNError::ValidationError(e.to_string()))?
            .unwrap();

        // 2. resolve odrl policy
        let odrl_ids = match &input.odrl_offer {
            ContractRequestMessageOfferTypes::OfferId(offer) => Ok::<_, anyhow::Error>(offer.id.clone()),
            ContractRequestMessageOfferTypes::OfferMessage(offer) => Ok::<_, anyhow::Error>(offer.id.clone()),
        }?;
        let offer = match self.catalog_facade.resolve_odrl_offers(odrl_ids).await {
            Ok(resolver) => resolver,
            Err(_) => bail!(IdsaCNError::NotCheckedError {
                provider_pid: None,
                consumer_pid: None,
                error: "Id not found".to_string()
            }),
        };
        match &input.odrl_offer {
            ContractRequestMessageOfferTypes::OfferMessage(input_offer) => {
                if input_offer.target.clone() != offer.target.clone().unwrap() {
                    bail!(IdsaCNError::NotCheckedError {
                        provider_pid: None,
                        consumer_pid: None,
                        error: "target not coincide".to_string()
                    })
                }
            }
            ContractRequestMessageOfferTypes::OfferId(_) => {}
        }

        // 3. persist process, message and offer
        let _ = self
            .repo
            .put_cn_process(
                get_urn_from_string(&cn_process.cn_process_id)?,
                EditContractNegotiationProcess {
                    provider_id: None, // no need to change
                    consumer_id: None,
                    state: Option::from(ContractNegotiationState::Offered),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        let cn_message = self
            .repo
            .create_cn_message(
                get_urn_from_string(&cn_process.cn_process_id)?,
                NewContractNegotiationMessage {
                    _type: input._type.to_string(),
                    from: "Consumer".to_string(),
                    to: "Provider".to_string(),
                    content: serde_json::to_value(&input).unwrap(),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        let offer = self
            .repo
            .create_cn_offer(
                get_urn_from_string(&cn_process.cn_process_id)?,
                get_urn_from_string(&cn_message.cn_message_id)?,
                NewContractNegotiationOffer { offer_id: None, offer_content: serde_json::to_value(&input.odrl_offer)? },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        // 4. notify subscriptions
        self.notify_subscribers(
            "ContractRequestMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
                "offer": offer
            }),
        )
            .await?;

        Ok(cn_process.into())
    }

    async fn post_provider_events(
        &self,
        provider_pid: Urn,
        input: ContractNegotiationEventMessage,
    ) -> anyhow::Result<ContractAckMessage> {
        // 1. validate request
        self.transition_validation(&input).await.map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let cn_process = self
            .payload_validation(Some(&provider_pid), &input)
            .await
            .map_err(|e| IdsaCNError::ValidationError(e.to_string()))?
            .unwrap();

        // 2. persist process, message and offer
        let cn_process = self
            .repo
            .put_cn_process(
                get_urn_from_string(&cn_process.cn_process_id)?,
                EditContractNegotiationProcess {
                    provider_id: None, // no need to change
                    consumer_id: None,
                    state: Some(input.event_type.clone().into()),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        let message = self
            .repo
            .create_cn_message(
                get_urn_from_string(&cn_process.cn_process_id)?,
                NewContractNegotiationMessage {
                    _type: input._type.to_string(),
                    from: "Consumer".to_string(),
                    to: "Provider".to_string(),
                    content: serde_json::to_value(&input.clone()).unwrap(),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        // 3. notify subscriptions
        self.notify_subscribers(
            "ContractNegotiationEventMessage:accepted".to_string(),
            json!({
                "process": cn_process,
                "message": message
            }),
        )
            .await?;
        Ok(cn_process.into())
    }

    async fn post_provider_agreement_verification(
        &self,
        provider_pid: Urn,
        input: ContractAgreementVerificationMessage,
    ) -> anyhow::Result<ContractAckMessage> {
        // 1. validate request
        self.transition_validation(&input).await.map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let cn_process = self
            .payload_validation(Some(&provider_pid), &input)
            .await
            .map_err(|e| IdsaCNError::ValidationError(e.to_string()))?
            .unwrap();
        let ContractAgreementVerificationMessage { _type, .. } = input.clone();

        // 2. persist process, message
        let cn_process = self
            .repo
            .put_cn_process(
                get_urn_from_string(&cn_process.cn_process_id)?,
                EditContractNegotiationProcess {
                    provider_id: None, // no need to change
                    consumer_id: None,
                    state: Some(ContractNegotiationState::Verified),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;
        // Create message
        let message = self
            .repo
            .create_cn_message(
                get_urn_from_string(&cn_process.cn_process_id)?,
                NewContractNegotiationMessage {
                    _type: _type.to_string(),
                    from: "Consumer".to_string(),
                    to: "Provider".to_string(),
                    content: serde_json::to_value(&input).unwrap(),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        // 3. notify subscriptions
        self.notify_subscribers(
            "ContractAgreementVerificationMessage".to_string(),
            json!({
                "process": cn_process,
                "message": message
            }),
        )
            .await?;
        Ok(cn_process.into())
    }

    async fn post_provider_termination(
        &self,
        provider_id: Urn,
        input: ContractTerminationMessage,
    ) -> anyhow::Result<ContractAckMessage> {
        // 1. validate request
        self.transition_validation(&input).await.map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let cn_process = self
            .payload_validation(Some(&provider_id), &input)
            .await
            .map_err(|e| IdsaCNError::ValidationError(e.to_string()))?
            .unwrap();

        // 2. persist process, message and offer
        let ContractTerminationMessage { _type, .. } = input.clone();
        let cn_process = self
            .repo
            .put_cn_process(
                get_urn_from_string(&cn_process.cn_process_id)?,
                EditContractNegotiationProcess {
                    provider_id: None, // no need to change
                    consumer_id: None,
                    state: Some(ContractNegotiationState::Terminated),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;
        // Create message
        let message = self
            .repo
            .create_cn_message(
                get_urn_from_string(&cn_process.cn_process_id)?,
                NewContractNegotiationMessage {
                    _type: _type.to_string(),
                    from: "Consumer".to_string(),
                    to: "Provider".to_string(),
                    content: serde_json::to_value(&input).unwrap(),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        // 3. notify subscriptions
        self.notify_subscribers(
            "ContractTerminationMessage".to_string(),
            json!({
                "process": cn_process,
                "message": message
            }),
        )
            .await?;
        Ok(cn_process.into())
    }
}
