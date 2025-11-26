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

use crate::common::schemas::validation::validate_payload_schema;
use crate::consumer::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use crate::consumer::core::ds_protocol::DSProtocolContractNegotiationConsumerTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::facades::ssi_auth_facade::SSIAuthFacadeTrait;
use rainbow_common::mates::Mates;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement::ContractAgreementMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{
    ContractNegotiationEventMessage, NegotiationEventType,
};
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_offer::ContractOfferMessage;
use rainbow_common::protocol::contract::contract_protocol_trait::DSProtocolContractNegotiationMessageTrait;
use rainbow_common::protocol::contract::{ContractNegotiationMessages, ContractNegotiationState};
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_consumer::entities::cn_process;
use rainbow_db::contracts_consumer::repo::{
    AgreementConsumerRepo, CnErrors, ContractNegotiationConsumerMessageRepo, ContractNegotiationConsumerOfferRepo,
    ContractNegotiationConsumerProcessRepo, NewAgreement, NewContractNegotiationProcess,
};
use rainbow_db::contracts_consumer::repo::{NewContractNegotiationMessage, NewContractNegotiationOffer};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, Value};
use std::fmt::Debug;
use std::sync::Arc;
use tracing::debug;
use urn::Urn;

pub struct DSProtocolContractNegotiationConsumerService<T, U, W>
where
    T: ContractNegotiationConsumerProcessRepo
        + ContractNegotiationConsumerMessageRepo
        + ContractNegotiationConsumerOfferRepo
        + AgreementConsumerRepo
        + Send
        + Sync
        + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    W: SSIAuthFacadeTrait + Sync + Send,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
    ssi_auth_facade: Arc<W>,
}

impl<T, U, W> DSProtocolContractNegotiationConsumerService<T, U, W>
where
    T: ContractNegotiationConsumerProcessRepo
        + ContractNegotiationConsumerMessageRepo
        + ContractNegotiationConsumerOfferRepo
        + AgreementConsumerRepo
        + Send
        + Sync
        + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    W: SSIAuthFacadeTrait + Sync + Send,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>, ssi_auth_facade: Arc<W>) -> Self {
        Self { repo, notification_service, ssi_auth_facade }
    }

    /// Validate auth token
    async fn validate_auth_token(&self, token: String) -> anyhow::Result<Mates> {
        let mate = self.ssi_auth_facade.verify_token(token).await?;
        Ok(mate)
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

    async fn payload_validation<'a, M: DSProtocolContractNegotiationMessageTrait<'a> + Debug>(
        &self,
        incoming_consumer_pid: Option<&Urn>,
        message: &M,
        provider_participant_mate: &Mates,
    ) -> anyhow::Result<Option<cn_process::Model>> {
        // ) -> anyhow::Result<()> {
        debug!("Contract negotiation payload_validation");

        // 1. if consumer is none, provider_pid shouldn't exist yet
        if incoming_consumer_pid.is_none() {
            let provider_pid = message.get_provider_pid()?.unwrap();
            match self.repo.get_cn_process_by_provider_id(provider_pid.to_owned()).await {
                Ok(cn) => match cn {
                    None => {} // // if process not found ok
                    Some(_) => bail!(IdsaCNError::ValidationError(format!(
                        "Provider {} pid already exists",
                        provider_pid
                    ))),
                },
                Err(e) => match e {
                    CnErrors::CNProcessNotFound => {} // if process not found ok
                    e_ => bail!(IdsaCNError::DbErr(e_)),
                },
            }
        }

        // 2. there must be process correlation between incoming_consumer_pid and consumer pid in body
        match (incoming_consumer_pid, message.get_consumer_pid()?) {
            (None, _) => {}
            (Some(i), Some(p)) if i == p => {}
            _out => bail!(IdsaCNError::ValidationError(
                "Consumer pid in body should coincide with URL path".to_string()
            )),
        }

        // 3. there must be process correlation between provider pid and consumer pid
        // Ack and Error don't need this validation
        match message.get_message_type()? {
            ContractNegotiationMessages::ContractNegotiationAck => Ok(None),
            ContractNegotiationMessages::ContractNegotiationError => Ok(None),
            ContractNegotiationMessages::ContractOfferMessage if incoming_consumer_pid.is_none() => Ok(None),
            _ => {
                let cn_process_consumer = self
                    .repo
                    .get_cn_process_by_consumer_id(message.get_consumer_pid()?.unwrap().to_owned())
                    .await
                    .map_err(IdsaCNError::DbErr)?
                    .ok_or(IdsaCNError::ProcessNotFound {
                        provider_pid: message.get_provider_pid()?.map(|p| p.to_owned()),
                        consumer_pid: Option::from(incoming_consumer_pid.unwrap().to_owned()),
                    })?;
                let cn_process_provider = self
                    .repo
                    .get_cn_process_by_provider_id(message.get_provider_pid()?.unwrap().to_owned())
                    .await
                    .map_err(IdsaCNError::DbErr)?
                    .ok_or(IdsaCNError::ProcessNotFound {
                        provider_pid: message.get_provider_pid()?.map(|p| p.to_owned()),
                        consumer_pid: Option::from(incoming_consumer_pid.unwrap().to_owned()),
                    })?;

                if cn_process_consumer.consumer_id != cn_process_provider.consumer_id {
                    bail!(IdsaCNError::ValidationError(
                        "ConsumerPid and ProviderPid don't coincide".to_string()
                    ))
                }

                // 4. Validate process is correlated with mate
                if cn_process_consumer.associated_provider.clone().unwrap() != provider_participant_mate.participant_id
                {
                    bail!(IdsaCNError::ValidationError(
                        "This user is not related with this process".to_string()
                    ))
                }
                Ok(Option::from(cn_process_consumer).map(|c| c.into()))
            }
        }
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
impl<T, U, W> DSProtocolContractNegotiationConsumerTrait for DSProtocolContractNegotiationConsumerService<T, U, W>
where
    T: ContractNegotiationConsumerProcessRepo
        + ContractNegotiationConsumerMessageRepo
        + ContractNegotiationConsumerOfferRepo
        + AgreementConsumerRepo
        + Send
        + Sync
        + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
    W: SSIAuthFacadeTrait + Sync + Send,
{
    async fn post_offers(&self, input: ContractOfferMessage, token: String) -> anyhow::Result<ContractAckMessage> {
        // 1. validate request
        let provider_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let _ = self
            .payload_validation(None, &input, &provider_participant_mate)
            .await
            .map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;

        // 2. persist
        let cn_process = self
            .repo
            .create_cn_process(NewContractNegotiationProcess {
                provider_id: Some(input.provider_pid.clone()),
                consumer_id: None,
                associated_provider: Some(provider_participant_mate.participant_id.clone()),
                is_business: false,
            })
            .await
            .map_err(IdsaCNError::DbErr)?;
        let cn_message = self
            .repo
            .create_cn_message(
                get_urn_from_string(&cn_process.consumer_id)?,
                NewContractNegotiationMessage {
                    _type: input._type.to_string(),
                    subtype: None,
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
                get_urn_from_string(&cn_process.consumer_id)?,
                get_urn_from_string(&cn_message.cn_message_id)?,
                NewContractNegotiationOffer { offer_id: None, offer_content: serde_json::to_value(&input.odrl_offer)? },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;
        // 3. prepare message
        let mut cn_ack: ContractAckMessage = cn_process.clone().into();
        cn_ack.state = ContractNegotiationState::Offered;

        // 4. notify
        self.notify_subscribers(
            "ContractOfferMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
                "offer": cn_offer
            }),
        )
        .await?;

        Ok(cn_ack)
    }

    async fn post_consumer_offers(
        &self,
        consumer_pid: Urn,
        input: ContractOfferMessage,
        token: String,
    ) -> anyhow::Result<ContractAckMessage> {
        // 1. validate request
        let provider_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let cn_process = self
            .payload_validation(Some(&consumer_pid), &input, &provider_participant_mate)
            .await
            .map_err(|e| IdsaCNError::ValidationError(e.to_string()))?
            .unwrap();

        // 2. Persist info
        let cn_message = self
            .repo
            .create_cn_message(
                consumer_pid.clone(),
                NewContractNegotiationMessage {
                    _type: input._type.to_string(),
                    subtype: None,
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
                consumer_pid,
                get_urn_from_string(&cn_message.cn_message_id)?,
                NewContractNegotiationOffer { offer_id: None, offer_content: serde_json::to_value(&input.odrl_offer)? },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        // 3. prepare message
        let mut cn_ack: ContractAckMessage = cn_process.clone().into();
        cn_ack.state = ContractNegotiationState::Offered;

        // 4. notify
        self.notify_subscribers(
            "ContractOfferMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
                "offer": cn_offer
            }),
        )
        .await?;

        Ok(cn_ack)
    }

    async fn post_agreement(
        &self,
        consumer_pid: Urn,
        input: ContractAgreementMessage,
        token: String,
    ) -> anyhow::Result<ContractAckMessage> {
        // 1. validate request
        let provider_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let cn_process = self
            .payload_validation(Some(&consumer_pid), &input, &provider_participant_mate)
            .await
            .map_err(|e| IdsaCNError::ValidationError(e.to_string()))?
            .unwrap();

        // 2. Persist info
        let cn_message = self
            .repo
            .create_cn_message(
                consumer_pid.clone(),
                NewContractNegotiationMessage {
                    _type: input._type.to_string(),
                    subtype: None,
                    from: "Consumer".to_string(),
                    to: "Provider".to_string(),
                    content: serde_json::to_value(&input).unwrap(),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;
        let agreement = self
            .repo
            .create_agreement(
                get_urn_from_string(&cn_process.consumer_id.clone())?,
                get_urn_from_string(&cn_message.cn_message_id.clone())?,
                NewAgreement {
                    agreement_id: Some(input.odrl_agreement.clone().id),
                    consumer_participant_id: input.odrl_agreement.clone().assignee,
                    provider_participant_id: input.odrl_agreement.clone().assigner,
                    agreement_content: input.odrl_agreement,
                    active: true,
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        // 3. prepare message
        let mut cn_ack: ContractAckMessage = cn_process.clone().into();
        cn_ack.state = ContractNegotiationState::Agreed;

        // 4. notify
        self.notify_subscribers(
            "ContractAgreementMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
                "agreement": agreement
            }),
        )
        .await?;

        Ok(cn_ack)
    }

    async fn post_events(
        &self,
        consumer_pid: Urn,
        input: ContractNegotiationEventMessage,
        token: String,
    ) -> anyhow::Result<ContractAckMessage> {
        // verify finalized
        if input.event_type != NegotiationEventType::Finalized {
            bail!(IdsaCNError::NotAllowed {
                provider_pid: None,
                consumer_pid: Option::from(consumer_pid),
                error: "Event must be FINALIZED type".to_string()
            });
        }
        // 1. validate request
        let provider_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let cn_process = self
            .payload_validation(Some(&consumer_pid), &input, &provider_participant_mate)
            .await
            .map_err(|e| IdsaCNError::ValidationError(e.to_string()))?
            .unwrap();

        // 2. Persist info
        let cn_message = self
            .repo
            .create_cn_message(
                consumer_pid.clone(),
                NewContractNegotiationMessage {
                    _type: input._type.to_string(),
                    subtype: Some(
                        match input.event_type {
                            NegotiationEventType::Accepted => "accepted",
                            NegotiationEventType::Finalized => "finalized",
                        }
                        .to_string(),
                    ),
                    from: "Consumer".to_string(),
                    to: "Provider".to_string(),
                    content: serde_json::to_value(&input).unwrap(),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        // 3. prepare message
        let mut cn_ack: ContractAckMessage = cn_process.clone().into();
        cn_ack.state = ContractNegotiationState::Finalized;

        // 4. notify
        self.notify_subscribers(
            "ContractEventMessage:finalized".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
            }),
        )
        .await?;

        Ok(cn_ack)
    }

    async fn post_termination(
        &self,
        consumer_pid: Urn,
        input: ContractTerminationMessage,
        token: String,
    ) -> anyhow::Result<ContractAckMessage> {
        // 1. validate request
        let provider_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input).map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;
        let cn_process = self
            .payload_validation(Some(&consumer_pid), &input, &provider_participant_mate)
            .await
            .map_err(|e| IdsaCNError::ValidationError(e.to_string()))?
            .unwrap();

        // 2. Persist info
        let cn_message = self
            .repo
            .create_cn_message(
                consumer_pid.clone(),
                NewContractNegotiationMessage {
                    _type: input._type.to_string(),
                    subtype: None,
                    from: "Consumer".to_string(),
                    to: "Provider".to_string(),
                    content: serde_json::to_value(&input).unwrap(),
                },
            )
            .await
            .map_err(IdsaCNError::DbErr)?;

        // 3. prepare message
        let mut cn_ack: ContractAckMessage = cn_process.clone().into();
        cn_ack.state = ContractNegotiationState::Terminated;

        // 4. notify
        self.notify_subscribers(
            "ContractTerminationMessage".to_string(),
            json!({
                "process": cn_process,
                "message": cn_message,
            }),
        )
        .await?;

        Ok(cn_ack)
    }
}
