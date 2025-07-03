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
use crate::common::utils::has_data_address_in_push;
use crate::consumer::core::ds_protocol::ds_protocol_err::DSProtocolTransferConsumerErrors;
use crate::provider::core::data_plane_facade::DataPlaneProviderFacadeTrait;
use crate::provider::core::data_service_resolver_facade::DataServiceFacadeTrait;
use crate::provider::core::ds_protocol::ds_protocol_err::DSProtocolTransferProviderErrors;
use crate::provider::core::ds_protocol::DSProtocolTransferProviderTrait;
use crate::provider::core::rainbow_entities::rainbow_err::RainbowTransferProviderErrors;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::err::transfer_err::TransferErrorType;
use rainbow_common::facades::ssi_auth_facade::SSIAuthFacadeTrait;
use rainbow_common::mates::Mates;
use rainbow_common::protocol::contract::contract_protocol_trait::DSProtocolContractNegotiationMessageTrait;
use rainbow_common::protocol::contract::ContractNegotiationMessages;
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_protocol_trait::DSProtocolTransferMessageTrait;
use rainbow_common::protocol::transfer::transfer_request::TransferRequestMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::protocol::transfer::{TransferMessageTypes, TransferRoles, TransferState, TransferStateAttribute};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_dataplane::coordinator::controller::DataPlaneControllerTrait;
use rainbow_db::transfer_provider::entities::transfer_process;
use rainbow_db::transfer_provider::entities::transfer_process::Model;
use rainbow_db::transfer_provider::repo::{
    EditTransferProcessModel, NewTransferMessageModel, NewTransferProcessModel, TransferProviderRepoErrors,
    TransferProviderRepoFactory,
};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::json;
use std::sync::Arc;
use tracing::debug;
use urn::Urn;

pub struct DSProtocolTransferProviderImpl<T, V, W, X>
where
    T: TransferProviderRepoFactory + Send + Sync,
// U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneProviderFacadeTrait + Send + Sync,
    W: RainbowEventsNotificationTrait + Sync + Send,
    X: SSIAuthFacadeTrait + Sync + Send,
{
    transfer_repo: Arc<T>,
    data_service_facade: Arc<dyn DataServiceFacadeTrait + Send + Sync>,
    data_plane: Arc<V>,
    notification_service: Arc<W>,
    ssi_auth_facade: Arc<X>,
}

impl<T, V, W, X> DSProtocolTransferProviderImpl<T, V, W, X>
where
    T: TransferProviderRepoFactory + Send + Sync,
// U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneProviderFacadeTrait + Send + Sync,
    W: RainbowEventsNotificationTrait + Sync + Send,
    X: SSIAuthFacadeTrait + Sync + Send,
{
    pub fn new(
        transfer_repo: Arc<T>,
        data_service_facade: Arc<dyn DataServiceFacadeTrait + Send + Sync>,
        data_plane: Arc<V>,
        notification_service: Arc<W>,
        ssi_auth_facade: Arc<X>,
    ) -> Self {
        Self { transfer_repo, data_service_facade, data_plane, notification_service, ssi_auth_facade }
    }


    /// Validate auth token
    async fn validate_auth_token(&self, token: String) -> anyhow::Result<Mates> {
        let mate = self.ssi_auth_facade
            .verify_token(token)
            .await?;
        Ok(mate)
    }


    ///
    ///
    fn json_schema_validation<'a, M: DSProtocolTransferMessageTrait<'a>>(&self, message: &M) -> anyhow::Result<()> {
        debug!("Transfer provider JSON schema validation");
        validate_payload_schema(message)?;
        Ok(())
    }

    ///
    ///
    async fn payload_validation<'a, M: DSProtocolTransferMessageTrait<'a>>(
        &self,
        provider_pid_from_uri: &Urn, // Provider PID from URL path
        message: &M,                 // The incoming message
        consumer_participant_mate: &Mates,
    ) -> anyhow::Result<transfer_process::Model> {
        debug!("Transfer provider payload validation");
        let message_provider_pid = message.get_provider_pid()?;
        let message_consumer_pid = message.get_consumer_pid()?;
        // 1. Validate provider_pid in message body matches provider_pid from URI
        match message_provider_pid {
            Some(msg_p_pid) if msg_p_pid == provider_pid_from_uri => {}
            _ => bail!(DSProtocolTransferProviderErrors::UriAndBodyIdentifiersDoNotCoincide),
        }
        // 2. Validate correlation in processes / Get existing process
        let provider_transfer_process = self
            .transfer_repo
            .get_transfer_process_by_provider(provider_pid_from_uri.clone())
            .await
            .map_err(DSProtocolTransferProviderErrors::DbErr)?
            .ok_or(DSProtocolTransferProviderErrors::TransferProcessNotFound {
                provider_pid: Some(provider_pid_from_uri.clone()),
                consumer_pid: message_consumer_pid.map(|m| m.to_owned()),
            })?;
        // 3. Validate consumer_pid in message body matches consumer_pid in DB (if applicable)
        if let Some(msg_c_pid) = message_consumer_pid {
            let consumer_transfer_process = self
                .transfer_repo
                .get_transfer_process_by_consumer(msg_c_pid.clone())
                .await
                .map_err(DSProtocolTransferProviderErrors::DbErr)?
                .ok_or(DSProtocolTransferProviderErrors::TransferProcessNotFound {
                    provider_pid: message_provider_pid.map(|m| m.to_owned()),
                    consumer_pid: Some(msg_c_pid.to_owned()),
                })?;
            if consumer_transfer_process.provider_pid != provider_transfer_process.provider_pid {
                bail!(RainbowTransferProviderErrors::ValidationError(
                    "ConsumerPid and ProviderPid don't coincide".to_string()
                ))
            }
            // 4. Validate process is correlated with mate
            if provider_transfer_process.associated_consumer.clone().unwrap() != consumer_participant_mate.participant_id {
                bail!(RainbowTransferProviderErrors::ValidationError(
                    "This user is not related with this process".to_string()
                ))
            }
        }
        Ok(provider_transfer_process)
    }

    ///
    ///
    async fn transition_validation<'a, M: DSProtocolTransferMessageTrait<'a>>(
        &self,
        message: &M,
    ) -> anyhow::Result<()> {
        // Negotiation state
        let consumer_pid = message.get_consumer_pid()?.to_owned();
        let provider_pid = message.get_provider_pid()?;
        let message_type = message.get_message_type()?;

        match message_type {
            TransferMessageTypes::TransferRequestMessage => {
                match (provider_pid, consumer_pid) {
                    // 1. Provider must be none in TransferRequestMessage
                    (None, Some(c)) => {
                        // 2. Consumer must not exist yet in TransferRequestMessa
                        match self.transfer_repo.get_transfer_process_by_consumer(c.to_owned()).await? {
                            Some(_) => bail!(TransferErrorType::ConsumerAlreadyRegisteredError),
                            None => {}
                        }
                    }
                    _ => bail!(TransferErrorType::MessageTypeNotAcceptedError),
                }
            }
            m => {
                // 3. provider must exist for the rest of messages (checked by serde)
                let provider_pid = message.get_provider_pid()?.unwrap().to_owned();
                // 3. For the rest of messages tp must exist (checked in previous method)
                let tp = self.transfer_repo.get_transfer_process_by_provider(provider_pid).await?.unwrap();
                let transfer_state = tp.state.parse()?;
                match m {
                    // 4. Transfer start transition check
                    TransferMessageTypes::TransferStartMessage => match transfer_state {
                        TransferState::REQUESTED => {}
                        TransferState::STARTED => {
                            bail!(TransferErrorType::ProtocolError {
                                state: TransferState::STARTED,
                                message_type: "Start message is not allowed in STARTED state".to_string(),
                            })
                        }
                        TransferState::SUSPENDED => {
                            // 5. Transfer state attribute check.
                            // Start from state suspended is only allowed if
                            let transfer_state_attribute = tp.state_attribute.unwrap().parse()?;
                            match transfer_state_attribute {
                                TransferStateAttribute::ByConsumer => {}
                                TransferStateAttribute::OnRequest => {}
                                TransferStateAttribute::ByProvider => bail!(TransferErrorType::ProtocolError {
                                        state: TransferState::STARTED,
                                        message_type: "State SUSPENDED was established by Provider, Consumer is not allowed to change it".to_string(),
                                    })
                            }
                        }
                        TransferState::COMPLETED => {
                            bail!(TransferErrorType::ProtocolError {
                                state: TransferState::COMPLETED,
                                message_type: "Start message is not allowed in COMPLETED state".to_string(),
                            })
                        }
                        TransferState::TERMINATED => {
                            bail!(TransferErrorType::ProtocolError {
                                state: TransferState::TERMINATED,
                                message_type: "Start message is not allowed in TERMINATED state".to_string(),
                            })
                        }
                    },
                    // 4. Transfer suspension transition check
                    TransferMessageTypes::TransferSuspensionMessage => {
                        match transfer_state {
                            TransferState::REQUESTED => {
                                bail!(TransferErrorType::ProtocolError {
                                    state: TransferState::REQUESTED,
                                    message_type: "Suspension message is not allowed in REQUESTED state".to_string(),
                                })
                            }
                            TransferState::STARTED => {}
                            TransferState::SUSPENDED => {
                                bail!(TransferErrorType::TransferProcessAlreadySuspendedError)
                            }
                            TransferState::COMPLETED => {
                                bail!(TransferErrorType::ProtocolError {
                                    state: TransferState::COMPLETED,
                                    message_type: "Suspension message is not allowed in COMPLETED state".to_string(),
                                })
                            }
                            TransferState::TERMINATED => {
                                bail!(TransferErrorType::ProtocolError {
                                    state: TransferState::TERMINATED,
                                    message_type: "Suspension message is not allowed in TERMINATED state".to_string(),
                                })
                            }
                        }
                    }
                    // 4. Transfer completion transition check
                    TransferMessageTypes::TransferCompletionMessage => match transfer_state {
                        TransferState::REQUESTED => {
                            bail!(TransferErrorType::ProtocolError {
                                state: TransferState::REQUESTED,
                                message_type: "Completion message is not allowed in REQUESTED state".to_string(),
                            })
                        }
                        TransferState::STARTED => {}
                        TransferState::SUSPENDED => {}
                        TransferState::COMPLETED => {}
                        TransferState::TERMINATED => {
                            bail!(TransferErrorType::ProtocolError {
                                state: TransferState::TERMINATED,
                                message_type: "Completion message is not allowed in TERMINATED state".to_string(),
                            })
                        }
                    },
                    // 4. Transfer termination transition check
                    TransferMessageTypes::TransferTerminationMessage => match transfer_state {
                        TransferState::REQUESTED => {}
                        TransferState::STARTED => {}
                        TransferState::SUSPENDED => {}
                        TransferState::COMPLETED => {
                            bail!(TransferErrorType::ProtocolError {
                                state: TransferState::COMPLETED,
                                message_type: "Termination message is not allowed in COMPLETED state".to_string(),
                            })
                        }
                        TransferState::TERMINATED => {}
                    },
                    // 4. Rest of messages not allowed
                    _ => bail!(TransferErrorType::MessageTypeNotAcceptedError),
                }
            }
        }

        Ok(())
    }

    ///
    ///
    async fn notify_subscribers(&self, subcategory: String, message: serde_json::Value) -> anyhow::Result<()> {
        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
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
impl<T, V, W, X> DSProtocolTransferProviderTrait for DSProtocolTransferProviderImpl<T, V, W, X>
where
    T: TransferProviderRepoFactory + Send + Sync,
// U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneProviderFacadeTrait + Send + Sync,
    W: RainbowEventsNotificationTrait + Sync + Send,
    X: SSIAuthFacadeTrait + Sync + Send,
{
    async fn get_transfer_requests_by_provider(&self, provider_pid: Urn) -> anyhow::Result<TransferProcessMessage> {
        debug!("DSProtocol Service: get_transfer_requests_by_provider");
        let transfers = self
            .transfer_repo
            .get_transfer_process_by_provider(provider_pid.clone())
            .await
            .map_err(DSProtocolTransferProviderErrors::DbErr)?
            .ok_or(DSProtocolTransferProviderErrors::TransferProcessNotFound {
                provider_pid: Option::from(provider_pid),
                consumer_pid: None,
            })?;
        Ok(transfers.into())
    }

    async fn get_transfer_requests_by_consumer(
        &self,
        consumer_pid: Urn,
    ) -> anyhow::Result<Option<TransferProcessMessage>> {
        debug!("DSProtocol Service: get_transfer_requests_by_consumer");
        let transfers = self
            .transfer_repo
            .get_transfer_process_by_consumer(consumer_pid.clone())
            .await
            .map_err(DSProtocolTransferProviderErrors::DbErr)?;
        Ok(transfers.map(|e| e.into()))
    }

    async fn transfer_request(&self, input: TransferRequestMessage, token: String) -> anyhow::Result<TransferProcessMessage> {
        debug!("DSProtocol Service: transfer_request");
        // 0. Extract data
        let provider_pid = get_urn(None);
        let consumer_pid = input.consumer_pid.to_owned();
        let agreement_id = get_urn_from_string(&input.agreement_id)?;
        let formats = input.format.clone();
        let _created_at = chrono::Utc::now().naive_utc();
        let message_type = input._type.clone();

        // 1. Validate request
        let consumer_participant_mate = self.validate_auth_token(token).await?;
        let callback_address = input.callback_address.clone().unwrap_or(consumer_participant_mate.base_url.unwrap_or("".to_string()));
        self.json_schema_validation(&input)
            .map_err(|e| RainbowTransferProviderErrors::ValidationError(e.to_string()))?;
        self.transition_validation(&input).await?;
        if !has_data_address_in_push(&input.data_address, &input.format)? {
            bail!(
                DSProtocolTransferProviderErrors::DataAddressCannotBeNullOnPushError {
                    consumer_pid: Some(consumer_pid.clone()),
                    provider_pid: None
                }
            );
        }

        // 2. Resolve data service
        let data_service =
            self.data_service_facade.resolve_data_service_by_agreement_id(agreement_id.clone(), Some(formats)).await?;
        // 3. Data plane hook
        self.data_plane.on_transfer_request(provider_pid.clone(), data_service, input.format.clone()).await?;

        // 4. Persist model
        let transfer_process = self
            .transfer_repo
            .create_transfer_process(NewTransferProcessModel {
                provider_pid: provider_pid.clone(),
                consumer_pid,
                agreement_id,
                callback_address, // TODO
                associated_consumer: Some(consumer_participant_mate.participant_id),
            })
            .await
            .map_err(DSProtocolTransferProviderErrors::DbErr)?;
        let transfer_message = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type: message_type.to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await
            .map_err(DSProtocolTransferProviderErrors::DbErr)?;
        // 5. Notify
        self.notify_subscribers(
            "TransferRequestMessage".to_string(),
            json!({
                "process": transfer_process.clone(),
                "message": transfer_message
            }),
        )
            .await?;
        // 6. Bye
        Ok(transfer_process.into())
    }

    async fn transfer_start(
        &self,
        provider_pid: Urn,
        input: TransferStartMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferStartMessage { provider_pid: provider_pid_, consumer_pid, .. } = input.clone();
        // 1. Validate request
        let consumer_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input)
            .map_err(|e| RainbowTransferProviderErrors::ValidationError(e.to_string()))?;
        self.transition_validation(&input).await?;
        let _ = self.payload_validation(&provider_pid, &input, &consumer_participant_mate).await?;
        // 2. Persist model/state
        let transfer_process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel {
                    state: Option::from(TransferState::STARTED),
                    state_attribute: Option::from(TransferStateAttribute::ByConsumer),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| match e {
                TransferProviderRepoErrors::ProviderTransferProcessNotFound => {
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid_.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    }
                }
                e_ => DSProtocolTransferProviderErrors::DbErr(e_),
            })?;
        // create message
        let transfer_message = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type: input._type.clone().to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await
            .map_err(DSProtocolTransferProviderErrors::DbErr)?;

        // 3. Data plane hook
        self.data_plane.on_transfer_start(provider_pid.clone()).await?;
        // 4. Notify
        self.notify_subscribers(
            "TransferStartMessage".to_string(),
            json!({
                "process": transfer_process.clone(),
                "message": transfer_message
            }),
        )
            .await?;
        // 5. Bye
        Ok(transfer_process.into())
    }

    async fn transfer_suspension(
        &self,
        provider_pid: Urn,
        input: TransferSuspensionMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferSuspensionMessage { provider_pid: provider_pid_, consumer_pid, .. } = input.clone();
        // 1. Validate request
        let consumer_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input)
            .map_err(|e| RainbowTransferProviderErrors::ValidationError(e.to_string()))?;
        let _ = self.payload_validation(&provider_pid, &input, &consumer_participant_mate).await?;
        self.transition_validation(&input).await?;
        // 2. Persist model/state
        let transfer_process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel {
                    state: Option::from(TransferState::SUSPENDED),
                    state_attribute: Option::from(TransferStateAttribute::ByConsumer),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| match e {
                TransferProviderRepoErrors::ProviderTransferProcessNotFound => {
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid_.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    }
                }
                e_ => DSProtocolTransferProviderErrors::DbErr(e_),
            })?;
        // create message
        let transfer_message = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type: input._type.clone().to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await
            .map_err(DSProtocolTransferProviderErrors::DbErr)?;
        // 3. Data plane hook
        self.data_plane.on_transfer_suspension(provider_pid.clone()).await?;
        // 4. Notify
        self.notify_subscribers(
            "TransferSuspensionMessage".to_string(),
            json!({
                "process": transfer_process.clone(),
                "message": transfer_message
            }),
        )
            .await?;
        // 5. Bye
        Ok(transfer_process.into())
    }

    async fn transfer_completion(
        &self,
        provider_pid: Urn,
        input: TransferCompletionMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferCompletionMessage { provider_pid: provider_pid_, consumer_pid, .. } = input.clone();
        // 1. Validate request
        let consumer_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input)
            .map_err(|e| RainbowTransferProviderErrors::ValidationError(e.to_string()))?;
        let _ = self.payload_validation(&provider_pid, &input, &consumer_participant_mate).await?;
        self.transition_validation(&input).await?;
        // 2. Persist model/state
        let transfer_process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel {
                    state: Option::from(TransferState::COMPLETED),
                    state_attribute: Option::from(TransferStateAttribute::ByConsumer),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| match e {
                TransferProviderRepoErrors::ProviderTransferProcessNotFound => {
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid_.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    }
                }
                e_ => DSProtocolTransferProviderErrors::DbErr(e_),
            })?;
        // create message
        let transfer_message = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type: input._type.clone().to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await
            .map_err(DSProtocolTransferProviderErrors::DbErr)?;
        // 3. Data plane hook
        self.data_plane.on_transfer_completion(provider_pid.clone()).await?;
        // 4. Notify
        self.notify_subscribers(
            "TransferCompletionMessage".to_string(),
            json!({
                "process": transfer_process.clone(),
                "message": transfer_message
            }),
        )
            .await?;
        // 5. Bye
        Ok(transfer_process.into())
    }

    async fn transfer_termination(
        &self,
        provider_pid: Urn,
        input: TransferTerminationMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferTerminationMessage { provider_pid: provider_pid_, consumer_pid, .. } = input.clone();
        // 1. Validate request
        let consumer_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input)
            .map_err(|e| RainbowTransferProviderErrors::ValidationError(e.to_string()))?;
        let _ = self.payload_validation(&provider_pid, &input, &consumer_participant_mate).await?;
        self.transition_validation(&input).await?;
        // 2. Persist model/state
        let transfer_process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel {
                    state: Option::from(TransferState::TERMINATED),
                    state_attribute: Option::from(TransferStateAttribute::ByConsumer),

                    ..Default::default()
                },
            )
            .await
            .map_err(|e| match e {
                TransferProviderRepoErrors::ProviderTransferProcessNotFound => {
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid_.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    }
                }
                e_ => DSProtocolTransferProviderErrors::DbErr(e_),
            })?;
        // create message
        let transfer_message = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type: input._type.clone().to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await
            .map_err(DSProtocolTransferProviderErrors::DbErr)?;
        // 3. Data plane hook
        self.data_plane.on_transfer_termination(provider_pid.clone()).await?;
        // 4. Notify
        self.notify_subscribers(
            "TransferTerminationMessage".to_string(),
            json!({
                "process": transfer_process.clone(),
                "message": transfer_message
            }),
        )
            .await?;
        // 5. Bye
        Ok(transfer_process.into())
    }
}
