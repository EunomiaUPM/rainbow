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
use crate::provider::core::data_plane_facade::DataPlaneProviderFacadeTrait;
use crate::provider::core::data_service_resolver_facade::DataServiceFacadeTrait;
use crate::provider::core::ds_protocol::ds_protocol_err::DSProtocolTransferProviderErrors;
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_err::DSRPCTransferProviderErrors;
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    DSRPCTransferProviderCompletionRequest, DSRPCTransferProviderCompletionResponse, DSRPCTransferProviderStartRequest,
    DSRPCTransferProviderStartResponse, DSRPCTransferProviderSuspensionRequest,
    DSRPCTransferProviderSuspensionResponse, DSRPCTransferProviderTerminationRequest,
    DSRPCTransferProviderTerminationResponse,
};
use crate::provider::core::ds_protocol_rpc::DSRPCTransferProviderTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::protocol::transfer::{TransferMessageTypes, TransferRoles, TransferState};
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
use tracing::debug;
use urn::Urn;

pub struct DSRPCTransferProviderService<T, U, V, W>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneProviderFacadeTrait + Send + Sync,
    W: RainbowEventsNotificationTrait + Sync + Send,
{
    transfer_repo: Arc<T>,
    _data_service_facade: Arc<U>,
    data_plane_facade: Arc<V>,
    notification_service: Arc<W>,
    client: Client,
}

impl<T, U, V, W> DSRPCTransferProviderService<T, U, V, W>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneProviderFacadeTrait + Send + Sync,
    W: RainbowEventsNotificationTrait + Sync + Send,
{
    pub fn new(
        transfer_repo: Arc<T>,
        _data_service_facade: Arc<U>,
        data_plane_facade: Arc<V>,
        notification_service: Arc<W>,
    ) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { transfer_repo, _data_service_facade, data_plane_facade, notification_service, client }
    }

    /// Validates the existence and correlation of provider and consumer transfer processes.
    async fn validate_and_get_correlated_transfer_process(
        &self,
        consumer_pid: &Urn,
        provider_pid: &Urn,
    ) -> anyhow::Result<transfer_process::Model> {
        debug!("{:?}", consumer_pid);
        debug!("{:?}", provider_pid);
        let provider_process = self
            .transfer_repo
            .get_transfer_process_by_provider(provider_pid.clone())
            .await
            .map_err(|e| {
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
            })?
            .ok_or(
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    },
                ),
            )?;
        // debug!("{:?}", provider_process);
        // debug!("{:?}", consumer_process);

        let consumer_process = self
            .transfer_repo
            .get_transfer_process_by_consumer(consumer_pid.clone())
            .await
            .map_err(|e| {
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
            })?
            .ok_or(
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    },
                ),
            )?;

        if provider_process.provider_pid != consumer_process.provider_pid {
            bail!(
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    }
                )
            );
        }
        Ok(provider_process)
    }

    /// Sends a protocol message to the consumer and handles the response.
    async fn send_protocol_message_to_consumer<M: serde::Serialize + std::fmt::Debug>(
        &self,
        target_url: String,
        message_payload: &M,
        error_context_provider_pid: Option<Urn>,
        error_context_consumer_pid: Option<Urn>,
    ) -> anyhow::Result<TransferProcessMessage> {
        debug!(
            "Sending message to consumer at URL: {}, Payload: {:?}",
            target_url, message_payload
        );
        let response = self.client.post(&target_url).json(message_payload).send().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerNotReachable {
                provider_pid: error_context_provider_pid.clone(),
                consumer_pid: error_context_consumer_pid.clone(),
            }
        })?;

        let status = response.status();
        if !status.is_success() {
            // Attempt to get error details from consumer if available
            let consumer_error = response.json::<Value>().await.unwrap_or_else(|e| json!({"error": format!("{}", e)}));
            bail!(DSRPCTransferProviderErrors::ConsumerInternalError {
                provider_pid: error_context_provider_pid.clone(),
                consumer_pid: error_context_consumer_pid.clone(),
                error: Some(consumer_error),
            });
        }

        let transfer_process_msg = response.json::<TransferProcessMessage>().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerResponseNotSerializable {
                provider_pid: error_context_provider_pid.clone(),
                consumer_pid: error_context_consumer_pid.clone(),
            }
        })?;
        Ok(transfer_process_msg)
    }

    /// Broadcasts a notification about a transfer process event.
    async fn notify_subscribers(&self, subcategory: String, message: Value) -> anyhow::Result<()> {
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
    U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneProviderFacadeTrait + Send + Sync,
    W: RainbowEventsNotificationTrait + Sync + Send,
{
    async fn setup_start(
        &self,
        input: DSRPCTransferProviderStartRequest,
    ) -> anyhow::Result<DSRPCTransferProviderStartResponse> {
        let DSRPCTransferProviderStartRequest { consumer_callback, provider_pid, consumer_pid, .. } = input;
        // 1. Validate fields and correlation
        let _ = self.validate_and_get_correlated_transfer_process(&consumer_pid, &provider_pid).await?;
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
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist transfer process state
        let process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel { state: Option::from(TransferState::STARTED), ..Default::default() },
            )
            .await
            .map_err(|e| {
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
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
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
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
            consumer_callback, provider_pid, consumer_pid, code, reason, ..
        } = input;
        // 1. Validate fields and correlation
        let _current_process_model =
            self.validate_and_get_correlated_transfer_process(&consumer_pid, &provider_pid).await?;
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
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist transfer process state
        let process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel { state: Option::from(TransferState::SUSPENDED), ..Default::default() },
            )
            .await
            .map_err(|e| {
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
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
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
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
        let DSRPCTransferProviderCompletionRequest { consumer_callback, provider_pid, consumer_pid, .. } = input;
        // 1. Validate fields and correlation
        let _current_process_model =
            self.validate_and_get_correlated_transfer_process(&consumer_pid, &provider_pid).await?;
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
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist transfer process state
        let process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel { state: Option::from(TransferState::COMPLETED), ..Default::default() },
            )
            .await
            .map_err(|e| {
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
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
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
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
            consumer_callback, provider_pid, consumer_pid, code, reason, ..
        } = input;
        // 1. Validate fields and correlation
        let _current_process_model =
            self.validate_and_get_correlated_transfer_process(&consumer_pid, &provider_pid).await?;
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
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist transfer process state
        let process = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel { state: Option::from(TransferState::TERMINATED), ..Default::default() },
            )
            .await
            .map_err(|e| {
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
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
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
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
