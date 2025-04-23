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

use crate::provider::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use crate::provider::core::ds_protocol::DSProtocolContractNegotiationProviderTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{ContractNegotiationEventMessage, NegotiationEventType};
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_odrl::OfferTypes;
use rainbow_common::protocol::contract::ContractNegotiationState;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_provider::repo::{AgreementRepo, ContractNegotiationMessageRepo, ContractNegotiationOfferRepo, ContractNegotiationProcessRepo, EditContractNegotiationProcess, NewContractNegotiationMessage, NewContractNegotiationOffer, NewContractNegotiationProcess, Participant};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::json;
use std::sync::Arc;
use urn::Urn;

pub struct DSProtocolContractNegotiationProviderService<T, U>
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
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> DSProtocolContractNegotiationProviderService<T, U>
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
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self { repo, notification_service }
    }
}

#[async_trait]
impl<T, U> DSProtocolContractNegotiationProviderTrait for DSProtocolContractNegotiationProviderService<T, U>
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
{
    async fn get_negotiation(&self, provider_pid: Urn) -> anyhow::Result<ContractAckMessage> {
        let cn_process = self.repo
            .get_cn_processes_by_provider_id(&provider_pid)
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Some(provider_pid),
                consumer_pid: None,
            })?;

        Ok(cn_process.into())
    }

    async fn post_request(&self, input: ContractRequestMessage) -> anyhow::Result<ContractAckMessage> {
        // Semantic Validate ContractRequestMessage
        // TODO may be in middleware
        input.validate().map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;

        // TODO Check schema compliance in middleware
        if let Some(provider_pid) = input.provider_pid.clone() {
            bail!(IdsaCNError::ValidationError(format!(
            "Provider PID is not allowed in request. Found: {}",
            provider_pid
        )));
        }

        // TODO Check transition protocol validations
        // Check if consumer_pid exists in the database
        // all this in middleware

        let cn_process = self.repo
            .create_cn_process(NewContractNegotiationProcess {
                provider_id: None,
                consumer_id: Option::from(input.consumer_pid.clone()),
                state: ContractNegotiationState::Requested,
                initiated_by: ConfigRoles::Consumer,
            })
            .await
            .map_err(IdsaCNError::DbErr)?;

        let cn_message = self.repo
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

        let offer = self.repo
            .create_cn_offer(
                get_urn_from_string(&cn_process.cn_process_id)?,
                get_urn_from_string(&cn_message.cn_message_id)?,
                NewContractNegotiationOffer {
                    offer_id: match input.odrl_offer {
                        OfferTypes::Offer(ref offer) => offer.id.clone(),
                        OfferTypes::MessageOffer(ref offer) => offer.id.clone(),
                        _ => bail!("Invalid offer type"),
                    },
                    offer_content: serde_json::to_value(&input.odrl_offer)?,
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
            subcategory: "ContractOfferMessage".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
            message_content: json!({
                "process": cn_process,
                "message": cn_message,
                "offer": offer
            }),
            message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
        }).await?;
        Ok(cn_process.into())
    }

    async fn post_provider_request(
        &self,
        provider_pid: Urn,
        input: ContractRequestMessage,
    ) -> anyhow::Result<ContractAckMessage> {
        // Semantic Validate ContractRequestMessage
        // TODO may be in middleware
        input.validate().map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;

        // Check if provider_pid in input and path match
        if let Some(provider_pid_in_input) = input.provider_pid.clone() {
            if provider_pid_in_input != provider_pid {
                bail!(IdsaCNError::ValidationError(format!(
                "Provider PID in path and in request body do not match. Path: {}, Request: {}",
                provider_pid, provider_pid_in_input
            )));
            }
        }

        // TODO Check transition protocol validations
        // Check if consumer_pid exists in the database
        // all this in middleware
        let _ = self.repo
            .get_cn_processes_by_consumer_id(input.consumer_pid.clone())
            .await?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Some(provider_pid.clone()),
                consumer_pid: Some(input.consumer_pid.clone()),
            })?;

        // Update CN process state and create message and offer
        let cn_process = self.repo
            .get_cn_processes_by_provider_id(&provider_pid)
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Some(provider_pid),
                consumer_pid: Some(input.consumer_pid.clone()),
            })?;
        let _ = self.repo
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

        let cn_message = self.repo
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

        let offer = self.repo
            .create_cn_offer(
                get_urn_from_string(&cn_process.cn_process_id)?,
                get_urn_from_string(&cn_message.cn_message_id)?,
                // TODO review this...
                NewContractNegotiationOffer {
                    offer_id: match input.odrl_offer {
                        OfferTypes::Offer(ref offer) => offer.id.clone(),
                        OfferTypes::MessageOffer(ref offer) => offer.id.clone(),
                        _ => bail!("Invalid offer type"),
                    },
                    offer_content: serde_json::to_value(&input.odrl_offer)?,
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
            subcategory: "ContractRequestMessage".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
            message_content: json!({
                "process": cn_process,
                "message": cn_message,
                "offer": offer
            }),
            message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
        }).await?;
        Ok(cn_process.into())
    }

    async fn post_provider_events(
        &self,
        provider_pid: Urn,
        input: ContractNegotiationEventMessage,
    ) -> anyhow::Result<ContractAckMessage> {
        let ContractNegotiationEventMessage { ref _type, ref consumer_pid, event_type, .. } = input;
        // Verify whether CN process was instantiated by the consumer
        let cn_process = self.repo
            .get_cn_processes_by_provider_id(&provider_pid)
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Some(provider_pid.clone()),
                consumer_pid: Some(consumer_pid.clone()),
            })?;

        match event_type {
            NegotiationEventType::Accepted => {}
            NegotiationEventType::Finalized => {
                return Err(IdsaCNError::NotAllowed {
                    provider_pid: Option::from(provider_pid.clone()),
                    consumer_pid: Some(consumer_pid.clone()),
                    error: "This message is not allowed".to_string(),
                }
                    .into())
            }
        };

        // let initiated_by = cn_process.initiated_by.parse::<ConfigRoles>().map_err(|e| {
        //     IdsaCNError::NotCheckedError {
        //         provider_pid: Option::from(provider_pid.clone().to_string()),
        //         consumer_pid: Some(input.consumer_pid.clone().to_string()),
        //         error: e.to_string(),
        //     }
        // })?;
        // match (initiated_by, event_type) {
        //     (ConfigRoles::Consumer, NegotiationEventType::Accepted) => {}
        //     _ => {
        //         return Err(IdsaCNError::NotAllowed {
        //             provider_pid: Option::from(provider_pid.clone()),
        //             consumer_pid: Some(input.consumer_pid.clone()),
        //             error: "This message is not allowed".to_string(),
        //         }
        //             .into())
        //     }
        // };

        // Update CN process state
        let cn_process = self.repo
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
        // Create message
        let message = self.repo
            .create_cn_message(
                get_urn_from_string(&cn_process.cn_process_id)?,
                NewContractNegotiationMessage {
                    _type: _type.to_string(),
                    from: "Consumer".to_string(),
                    to: "Provider".to_string(),
                    content: serde_json::to_value(&input.clone()).unwrap(),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
            subcategory: "ContractNegotiationEventMessage:accepted".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
            message_content: json!({
                "process": cn_process,
                "message": message
            }),
            message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
        }).await?;
        Ok(cn_process.into())
    }

    async fn post_provider_agreement_verification(
        &self,
        provider_id: Urn,
        input: ContractAgreementVerificationMessage,
    ) -> anyhow::Result<ContractAckMessage> {
        let ContractAgreementVerificationMessage { provider_pid, consumer_pid, _type, .. } =
            input.clone();
        let cn_process = self.repo
            .get_cn_processes_by_provider_id(&provider_id)
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Some(provider_pid.clone().parse()?),
                consumer_pid: Some(consumer_pid.clone().parse()?),
            })?;

        let cn_process = self.repo
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
        let message = self.repo
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

        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
            subcategory: "ContractAgreementVerificationMessage".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
            message_content: json!({
                "process": cn_process,
                "message": message
            }),
            message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
        }).await?;
        Ok(cn_process.into())
    }

    async fn post_provider_termination(
        &self,
        provider_id: Urn,
        input: ContractTerminationMessage,
    ) -> anyhow::Result<ContractAckMessage> {
        let ContractTerminationMessage { provider_pid, consumer_pid, _type, .. } = input.clone();
        let cn_process = self.repo
            .get_cn_processes_by_provider_id(&provider_id)
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Some(provider_pid.clone()),
                consumer_pid: Some(consumer_pid.clone()),
            })?;

        let cn_process = self.repo
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
        let message = self.repo
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

        self.notification_service.broadcast_notification(RainbowEventsNotificationBroadcastRequest {
            category: RainbowEventsNotificationMessageCategory::ContractNegotiation,
            subcategory: "ContractNegotiationTerminationMessage".to_string(),
            message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
            message_content: json!({
                "process": cn_process,
                "message": message
            }),
            message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
        }).await?;
        Ok(cn_process.into())
    }
}
