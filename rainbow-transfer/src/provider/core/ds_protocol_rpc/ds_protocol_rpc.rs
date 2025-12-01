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

use crate::common::errors::transfer_errors::TransferErrors;
use crate::provider::core::data_plane_facade::DataPlaneProviderFacadeTrait;
use crate::provider::core::data_service_resolver_facade::DataServiceFacadeTrait;
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    DSRPCTransferProviderCompletionRequest, DSRPCTransferProviderCompletionResponse, DSRPCTransferProviderStartRequest,
    DSRPCTransferProviderStartResponse, DSRPCTransferProviderSuspensionRequest,
    DSRPCTransferProviderSuspensionResponse, DSRPCTransferProviderTerminationRequest,
    DSRPCTransferProviderTerminationResponse,
};
use crate::provider::core::ds_protocol_rpc::DSRPCTransferProviderTrait;
use anyhow::{anyhow, bail};
use axum::async_trait;
use rainbow_common::err::transfer_err::TransferErrorType;
use rainbow_common::errors::helpers::{BadFormat, MissingAction};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::mates::Mates;
use rainbow_common::mates_facade::MatesFacadeTrait;
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_protocol_trait::DSProtocolTransferMessageTrait;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::protocol::transfer::{TransferMessageTypes, TransferRoles, TransferState, TransferStateAttribute};
use rainbow_db::transfer_provider::entities::transfer_process;
use rainbow_db::transfer_provider::repo::{
    EditTransferProcessModel, NewTransferMessageModel, TransferProviderRepoFactory,
};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use reqwest::Client;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error};
use urn::Urn;

pub struct DSRPCTransferProviderService<T, U, V, W>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataPlaneProviderFacadeTrait + Send + Sync,
    V: RainbowEventsNotificationTrait + Sync + Send,
    W: MatesFacadeTrait + Send + Sync,
{
    transfer_repo: Arc<T>,
    _data_service_facade: Arc<dyn DataServiceFacadeTrait + Send + Sync>,
    data_plane_facade: Arc<U>,
    notification_service: Arc<V>,
    client: Client,
    mates_facade: Arc<W>,
}

impl<T, U, V, W> DSRPCTransferProviderService<T, U, V, W>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataPlaneProviderFacadeTrait + Send + Sync,
    V: RainbowEventsNotificationTrait + Sync + Send,
    W: MatesFacadeTrait + Send + Sync,
{
    pub fn new(
        transfer_repo: Arc<T>,
        _data_service_facade: Arc<dyn DataServiceFacadeTrait + Send + Sync>,
        data_plane_facade: Arc<U>,
        notification_service: Arc<V>,
        mates_facade: Arc<W>,
    ) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { transfer_repo, _data_service_facade, data_plane_facade, notification_service, client, mates_facade }
    }

    /// Get consumer mate based in id
    async fn get_consumer_mate(&self, consumer_participant_id: &String) -> anyhow::Result<Mates> {
        debug!("DSProtocolRPC Service: get_consumer_mate");

        let mate = self.mates_facade.get_mate_by_id(consumer_participant_id.clone()).await.map_err(|e| {
            let e = CommonErrors::format_new(BadFormat::Received, &e.to_string());
            error!("{}", e.log());
            anyhow!(e)
        })?;
        Ok(mate)
    }

    /// Validates the existence and correlation of provider and consumer transfer processes.
    async fn validate_and_get_correlated_transfer_process(
        &self,
        consumer_pid: &Urn,
        provider_pid: &Urn,
    ) -> anyhow::Result<transfer_process::Model> {
        debug!("DSProtocolRPC Service: validate_and_get_correlated_transfer_process");

        let provider_process = self
            .transfer_repo
            .get_transfer_process_by_provider(provider_pid.clone())
            .await
            .map_err(|e| {
                let e = CommonErrors::format_new(BadFormat::Received, &e.to_string());
                error!("{}", e.log());
                anyhow!(e)
            })?
            .ok_or_else(|| {
                let e = CommonErrors::missing_resource_new(&provider_pid.to_string(), "Transfer process doesn't exist");
                error!("{}", e.log());
                anyhow!(e)
            })?;

        let consumer_process = self
            .transfer_repo
            .get_transfer_process_by_consumer(consumer_pid.clone())
            .await
            .map_err(|e| {
                let e = CommonErrors::format_new(BadFormat::Received, &e.to_string());
                error!("{}", e.log());
                anyhow!(e)
            })?
            .ok_or_else(|| {
                let e = CommonErrors::missing_resource_new(&consumer_pid.to_string(), "Transfer process doesn't exist");
                error!("{}", e.log());
                anyhow!(e)
            })?;

        if provider_process.provider_pid != consumer_process.provider_pid {
            let e = CommonErrors::format_new(
                BadFormat::Received,
                "ConsumerPid and ProviderPid don't coincide",
            );
            error!("{}", e.log());
            bail!(e);
        }
        Ok(provider_process)
    }

    async fn transition_validation<'a, M: DSProtocolTransferMessageTrait<'a>>(
        &self,
        message: &M,
    ) -> anyhow::Result<()> {
        debug!("DSProtocolRPC Service: transition_validation");

        // Negotiation state
        // For RPC consumer_pid and provider_pid are always some
        let _consumer_pid = message.get_consumer_pid()?.unwrap().to_owned();
        let provider_pid = message.get_provider_pid()?.unwrap().to_owned();
        let message_type = message.get_message_type()?;
        // For RPC transfer process is always there
        let tp = self.transfer_repo.get_transfer_process_by_provider(provider_pid).await?.unwrap();
        let transfer_state = tp.state.parse()?;
        let transfer_state_attribute =
            tp.state_attribute.unwrap_or(TransferStateAttribute::OnRequest.to_string()).parse()?;

        match message_type {
            TransferMessageTypes::TransferStartMessage => match transfer_state {
                TransferState::REQUESTED => {}
                TransferState::STARTED => {
                    let e = TransferErrors::protocol_new(
                        "Start message is not allowed in STARTED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    bail!(e);
                }
                TransferState::SUSPENDED => {
                    // Transfer state attribute check.
                    match transfer_state_attribute {
                        // If suspended by consumer, not able to start from provider
                        TransferStateAttribute::ByConsumer => {
                            let e = TransferErrors::protocol_new(
                                "State SUSPENDED was established by Consumer, Provider is not allowed to change it"
                                    .to_string()
                                    .into(),
                            );
                            error!("{}", e.log());
                            bail!(e);
                        }
                        TransferStateAttribute::OnRequest => {}
                        TransferStateAttribute::ByProvider => {}
                    }
                }
                TransferState::COMPLETED => {
                    let e = TransferErrors::protocol_new(
                        "Start message is not allowed in COMPLETED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    bail!(e);
                }
                TransferState::TERMINATED => {
                    let e = TransferErrors::protocol_new(
                        "Start message is not allowed in TERMINATED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    bail!(e);
                }
            },
            // 4. Transfer suspension transition check
            TransferMessageTypes::TransferSuspensionMessage => match transfer_state {
                TransferState::REQUESTED => {
                    let e = TransferErrors::protocol_new(
                        "Suspension message is not allowed in REQUESTED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    bail!(e);
                }
                TransferState::STARTED => {}
                TransferState::SUSPENDED => {
                    let e = TransferErrors::protocol_new("Transfer already suspended".to_string().into());
                    error!("{}", e.log());
                    bail!(e);
                }
                TransferState::COMPLETED => {
                    let e = TransferErrors::protocol_new(
                        "Suspension message is not allowed in COMPLETED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    bail!(e);
                }
                TransferState::TERMINATED => {
                    let e = TransferErrors::protocol_new(
                        "Suspension message is not allowed in TERMINATED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    bail!(e);
                }
            },
            // 4. Transfer completion transition check
            TransferMessageTypes::TransferCompletionMessage => match transfer_state {
                TransferState::REQUESTED => {
                    let e = TransferErrors::protocol_new(
                        "Completion message is not allowed in REQUESTED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    bail!(e);
                }
                TransferState::STARTED => {}
                TransferState::SUSPENDED => {}
                TransferState::COMPLETED => {}
                TransferState::TERMINATED => {
                    let e = TransferErrors::protocol_new(
                        "Completion message is not allowed in TERMINATED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    bail!(e);
                }
            },
            // 4. Transfer termination transition check
            TransferMessageTypes::TransferTerminationMessage => match transfer_state {
                TransferState::REQUESTED => {}
                TransferState::STARTED => {}
                TransferState::SUSPENDED => {}
                TransferState::COMPLETED => {
                    let e = TransferErrors::protocol_new(
                        "Completion message is not allowed in COMPLETED state".to_string().into(),
                    );
                    error!("{}", e.log());
                    bail!(e);
                }
                TransferState::TERMINATED => {}
            },
            // 4. Rest of messages not allowed
            _ => {
                let e = TransferErrors::protocol_new("This message type is not allowed".to_string().into());
                error!("{}", e.log());
                bail!(e);
            }
        }
        Ok(())
    }

    /// Sends a protocol message to the consumer and handles the response.
    async fn send_protocol_message_to_consumer<M: serde::Serialize + std::fmt::Debug>(
        &self,
        target_url: String,
        message_payload: &M,
        token: String,
        error_context_provider_pid: Option<Urn>,
        error_context_consumer_pid: Option<Urn>,
    ) -> anyhow::Result<TransferProcessMessage> {
        debug!("DSProtocolRPC Service: send_protocol_message_to_consumer");

        let response = self
            .client
            .post(&target_url)
            .header("Authorization", format!("Bearer {}", token))
            .json(message_payload)
            .send()
            .await
            .map_err(|_e| {
                let e = CommonErrors::consumer_new(&target_url, "POST", None, "Consumer not reachable");
                error!("{}", e.log());
                anyhow!(e)
            })?;

        let status = response.status();
        if !status.is_success() {
            // Attempt to get error details from consumer if available
            let consumer_error = response.json::<Value>().await.unwrap_or_else(|e| json!({"error": format!("{}", e)}));
            let e = CommonErrors::consumer_new(
                &target_url,
                "POST",
                None,
                &format!("Consumer Internal error: {}", consumer_error),
            );
            error!("{}", e.log());
            bail!(e);
        }

        let transfer_process_msg = response.json::<TransferProcessMessage>().await.map_err(|_e| {
            let e = CommonErrors::format_new(
                BadFormat::Received,
                "TransferProcessMessage not serializable",
            );
            error!("{}", e.log());
            anyhow!(e)
        })?;
        Ok(transfer_process_msg)
    }

    /// Broadcasts a notification about a transfer process event.
    async fn notify_subscribers(&self, subcategory: String, message: Value) -> anyhow::Result<()> {
        debug!("DSProtocolRPC Service: notify_subscribers");

        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory,
                message_type: RainbowEventsNotificationMessageTypes::RPCMessage,
                message_content: message,
                message_operation: RainbowEventsNotificationMessageOperation::OutgoingMessage,
            })
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<T, U, V, W> DSRPCTransferProviderTrait for DSRPCTransferProviderService<T, U, V, W>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataPlaneProviderFacadeTrait + Send + Sync,
    V: RainbowEventsNotificationTrait + Sync + Send,
    W: MatesFacadeTrait + Send + Sync,
{
    async fn setup_start(
        &self,
        input: DSRPCTransferProviderStartRequest,
    ) -> anyhow::Result<DSRPCTransferProviderStartResponse> {
        debug!("DSProtocolRPC Service: setup_start");

        let DSRPCTransferProviderStartRequest {
            consumer_participant_id,
            consumer_callback,
            provider_pid,
            consumer_pid,
            ..
        } = input.clone();
        // 0. fetch participant
        let consumer_mate = self.get_consumer_mate(&consumer_participant_id).await?;
        let consumer_callback = consumer_callback.unwrap_or(consumer_mate.base_url.unwrap());
        let consumer_token = consumer_mate.token.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Token, &"No auth token");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        // 1. Validate fields and correlation
        let tp = self.validate_and_get_correlated_transfer_process(&consumer_pid, &provider_pid).await?;
        self.transition_validation(&input).await?;
        // 2. Create message
        let data_address = self.data_plane_facade.get_dataplane_address(provider_pid.clone()).await?;
        let start_message = TransferStartMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            data_address: Some(data_address.clone()),
            ..Default::default()
        };
        // 3. Send message to consumer
        let consumer_callback = consumer_callback.strip_suffix('/').unwrap_or(consumer_callback.as_str());
        let consumer_url = format!(
            "{}/transfers/{}/start",
            consumer_callback,
            consumer_pid.clone()
        );
        let response = self
            .send_protocol_message_to_consumer(
                consumer_url,
                &start_message,
                consumer_token,
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist transfer process state
        // 4.1 is start ON_REQUEST OR BY_PROVIDER
        let attribute = match tp.state_attribute {
            None => TransferStateAttribute::OnRequest,
            Some(_) => TransferStateAttribute::ByProvider,
        };
        let process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel {
                    state: Option::from(TransferState::STARTED),
                    state_attribute: Option::from(attribute),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // persist message
        let message = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferStartMessage.to_string(),
                    from: TransferRoles::Provider,
                    to: TransferRoles::Consumer,
                    content: serde_json::to_value(start_message).unwrap(),
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // 5. Data plane facade hook
        self.data_plane_facade.on_transfer_start(provider_pid.clone()).await?;
        // 6. Create response
        let response = DSRPCTransferProviderStartResponse {
            provider_pid,
            consumer_pid,
            data_address: Some(data_address),
            message: response,
        };
        // 7. Notify subscribers
        self.notify_subscribers(
            "TransferStartMessage".to_string(),
            json!({
                "process": process,
                "message": message
            }),
        )
        .await?;
        // 8. Bye
        Ok(response)
    }

    async fn setup_suspension(
        &self,
        input: DSRPCTransferProviderSuspensionRequest,
    ) -> anyhow::Result<DSRPCTransferProviderSuspensionResponse> {
        let DSRPCTransferProviderSuspensionRequest {
            consumer_participant_id,
            consumer_callback,
            provider_pid,
            consumer_pid,
            code,
            reason,
            ..
        } = input.clone();
        // 0. fetch participant
        let consumer_mate = self.get_consumer_mate(&consumer_participant_id).await?;
        let consumer_callback = consumer_callback.unwrap_or(consumer_mate.base_url.unwrap());
        let consumer_token = consumer_mate.token.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Token, "No auth token");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        // 1. Validate fields and correlation
        let _current_process_model =
            self.validate_and_get_correlated_transfer_process(&consumer_pid, &provider_pid).await?;
        self.transition_validation(&input).await?;
        // 2. Create message
        let suspension_message = TransferSuspensionMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            code: code.clone(),
            reason: reason.clone(),
            ..Default::default()
        };
        // 3. Send message to consumer
        let consumer_callback = consumer_callback.strip_suffix('/').unwrap_or(consumer_callback.as_str());
        let consumer_url = format!(
            "{}/transfers/{}/suspension",
            consumer_callback,
            consumer_pid.clone()
        );
        let response = self
            .send_protocol_message_to_consumer(
                consumer_url,
                &suspension_message,
                consumer_token,
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist transfer process state
        let process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel {
                    state: Option::from(TransferState::SUSPENDED),
                    state_attribute: Option::from(TransferStateAttribute::ByProvider),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // persist message
        let message = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
                    from: TransferRoles::Provider,
                    to: TransferRoles::Consumer,
                    content: serde_json::to_value(suspension_message).unwrap(),
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // 5. Data plane facade hook
        self.data_plane_facade.on_transfer_suspension(provider_pid.clone()).await?;
        // 6. Create response
        let response = DSRPCTransferProviderSuspensionResponse { provider_pid, consumer_pid, message: response };
        // 7. Notify subscribers
        self.notify_subscribers(
            "TransferSuspensionMessage".to_string(),
            json!({
                "process": process,
                "message": message
            }),
        )
        .await?;
        // 8. Bye
        Ok(response)
    }

    async fn setup_completion(
        &self,
        input: DSRPCTransferProviderCompletionRequest,
    ) -> anyhow::Result<DSRPCTransferProviderCompletionResponse> {
        let DSRPCTransferProviderCompletionRequest {
            consumer_participant_id,
            consumer_callback,
            provider_pid,
            consumer_pid,
            ..
        } = input.clone();
        // 0. fetch participant
        let consumer_mate = self.get_consumer_mate(&consumer_participant_id).await?;
        let consumer_callback = consumer_callback.unwrap_or(consumer_mate.base_url.unwrap());
        let consumer_token = consumer_mate.token.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Token, "No auth token");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        // 1. Validate fields and correlation
        let _current_process_model =
            self.validate_and_get_correlated_transfer_process(&consumer_pid, &provider_pid).await?;
        self.transition_validation(&input).await?;
        // 2. Create message
        let completion_message = TransferCompletionMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            ..Default::default()
        };
        // 3. Send message to consumer
        let consumer_callback = consumer_callback.strip_suffix('/').unwrap_or(consumer_callback.as_str());
        let consumer_url = format!(
            "{}/transfers/{}/completion",
            consumer_callback,
            consumer_pid.clone()
        );
        let response = self
            .send_protocol_message_to_consumer(
                consumer_url,
                &completion_message,
                consumer_token,
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist transfer process state
        let process = self
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
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // persist message
        let message = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferCompletionMessage.to_string(),
                    from: TransferRoles::Provider,
                    to: TransferRoles::Consumer,
                    content: serde_json::to_value(completion_message).unwrap(),
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // 6. Data plane facade hook
        self.data_plane_facade.on_transfer_completion(provider_pid.clone()).await?;
        // 7. Create response
        let response = DSRPCTransferProviderCompletionResponse { provider_pid, consumer_pid, message: response };
        // 8. Notify subscribers
        self.notify_subscribers(
            "TransferCompletionMessage".to_string(),
            json!({
                "process": process,
                "message": message
            }),
        )
        .await?;
        // 9. Bye
        Ok(response)
    }

    async fn setup_termination(
        &self,
        input: DSRPCTransferProviderTerminationRequest,
    ) -> anyhow::Result<DSRPCTransferProviderTerminationResponse> {
        let DSRPCTransferProviderTerminationRequest {
            consumer_participant_id,
            consumer_callback,
            provider_pid,
            consumer_pid,
            code,
            reason,
            ..
        } = input.clone();
        // 0. fetch participant
        let consumer_mate = self.get_consumer_mate(&consumer_participant_id).await?;
        let consumer_callback = consumer_callback.unwrap_or(consumer_mate.base_url.unwrap());
        let consumer_token = consumer_mate.token.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Token, "No auth token");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        // 1. Validate fields and correlation
        let _current_process_model =
            self.validate_and_get_correlated_transfer_process(&consumer_pid, &provider_pid).await?;
        self.transition_validation(&input).await?;
        // 2. Create message
        let termination_message = TransferTerminationMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            code: code.clone(),
            reason: reason.clone(),
            ..Default::default()
        };
        // 3. Send message to consumer
        let consumer_callback = consumer_callback.strip_suffix('/').unwrap_or(consumer_callback.as_str());
        let consumer_url = format!(
            "{}/transfers/{}/termination",
            consumer_callback,
            consumer_pid.clone()
        );
        let response_message = self
            .send_protocol_message_to_consumer(
                consumer_url,
                &termination_message,
                consumer_token,
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist transfer process state
        let process = self
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
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // persist message
        let message = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferTerminationMessage.to_string(),
                    from: TransferRoles::Provider,
                    to: TransferRoles::Consumer,
                    content: serde_json::to_value(termination_message).unwrap(),
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // 5. Data plane facade hook
        self.data_plane_facade.on_transfer_termination(provider_pid.clone()).await?;
        // 6. Create response
        let response =
            DSRPCTransferProviderTerminationResponse { provider_pid, consumer_pid, message: response_message };
        // 7. Notify subscribers
        self.notify_subscribers(
            "TransferTerminationMessage".to_string(),
            json!({
                "process": process,
                "message": message
            }),
        )
        .await?;
        // 8. Bye
        Ok(response)
    }
}
