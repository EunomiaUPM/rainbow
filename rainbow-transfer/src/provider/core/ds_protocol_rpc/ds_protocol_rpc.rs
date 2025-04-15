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
use rainbow_db::transfer_provider::repo::{
    EditTransferProcessModel, NewTransferMessageModel, TransferProviderRepoFactory,
};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;

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
    pub fn new(transfer_repo: Arc<T>, _data_service_facade: Arc<U>, data_plane_facade: Arc<V>, notification_service: Arc<W>) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { transfer_repo, _data_service_facade, data_plane_facade, notification_service, client }
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
        let DSRPCTransferProviderStartRequest { consumer_callback, provider_pid, consumer_pid, data_address, .. } =
            input;
        // validate fields
        let provider = self
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
        let consumer = self
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
        // validate correlation
        if provider.provider_pid != consumer.provider_pid {
            bail!(
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    }
                )
            );
        };
        // create message
        let start_message = TransferStartMessage {
            provider_pid: provider_pid.clone().to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            data_address: data_address.clone(),
            ..Default::default()
        };
        // http to consumer
        // TODO participants...
        let consumer_callback = consumer_callback.strip_suffix('/').unwrap_or(consumer_callback.as_str());
        let consumer_url = format!(
            "{}/transfers/{}/start",
            consumer_callback,
            consumer_pid.clone()
        );
        let req = self.client.post(consumer_url).json(&start_message).send().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            }
        })?;

        // process response
        let status = req.status();
        if status.clone().is_success() == false {
            bail!(DSRPCTransferProviderErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }
        // parse response
        let response = req.json::<TransferProcessMessage>().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            }
        })?;
        // persist transfer process
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
        self.data_plane_facade.on_transfer_start().await?;
        let response = DSRPCTransferProviderStartResponse {
            provider_pid,
            consumer_pid,
            data_address: data_address,
            message: response,
        };
        self.notification_service.broadcast_notification(
            RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory: "TransferStartMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RPCMessage,
                message_operation: RainbowEventsNotificationMessageOperation::OutgoingMessage,
                message_content: json!({
                    "process": &process,
                    "message": &message
                }),
            }
        ).await?;
        Ok(response)
    }

    async fn setup_suspension(
        &self,
        input: DSRPCTransferProviderSuspensionRequest,
    ) -> anyhow::Result<DSRPCTransferProviderSuspensionResponse> {
        let DSRPCTransferProviderSuspensionRequest { consumer_callback, provider_pid, consumer_pid, code, reason, .. } =
            input;
        // validate fields
        let provider = self
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
        let consumer = self
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
        // validate correlation
        if provider.provider_pid != consumer.provider_pid {
            bail!(
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    }
                )
            );
        };
        // create message
        let start_message = TransferSuspensionMessage {
            provider_pid: provider_pid.clone().to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            code: code.clone(),
            reason: reason.clone(),
            ..Default::default()
        };
        // http to consumer
        // TODO participants...
        let consumer_callback = consumer_callback.strip_suffix('/').unwrap_or(consumer_callback.as_str());
        let consumer_url = format!(
            "{}/transfers/{}/suspension",
            consumer_callback,
            consumer_pid.clone()
        );
        let req = self.client.post(consumer_url).json(&start_message).send().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            }
        })?;
        // process response
        let status = req.status();
        if status.clone().is_success() == false {
            bail!(DSRPCTransferProviderErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }
        // parse response
        let response = req.json::<TransferProcessMessage>().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            }
        })?;
        // persist transfer process
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
                    content: serde_json::to_value(start_message).unwrap(),
                },
            )
            .await
            .map_err(|e| {
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
            })?;
        self.data_plane_facade.on_transfer_suspension().await?;
        let response = DSRPCTransferProviderSuspensionResponse {
            provider_pid,
            consumer_pid,
            message: response,
        };
        self.notification_service.broadcast_notification(
            RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory: "TransferSuspensionMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RPCMessage,
                message_operation: RainbowEventsNotificationMessageOperation::OutgoingMessage,
                message_content: json!({
                    "process": &process,
                    "message": &message
                }),
            }
        ).await?;
        Ok(response)
    }

    async fn setup_completion(
        &self,
        input: DSRPCTransferProviderCompletionRequest,
    ) -> anyhow::Result<DSRPCTransferProviderCompletionResponse> {
        let DSRPCTransferProviderCompletionRequest { consumer_callback, provider_pid, consumer_pid, .. } =
            input;
        // validate fields
        let provider = self
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
        let consumer = self
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
        // validate correlation
        if provider.provider_pid != consumer.provider_pid {
            bail!(
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    }
                )
            );
        };
        // create message
        let start_message = TransferCompletionMessage {
            provider_pid: provider_pid.clone().to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            ..Default::default()
        };
        // http to consumer
        // TODO participants...
        let consumer_callback = consumer_callback.strip_suffix('/').unwrap_or(consumer_callback.as_str());
        let consumer_url = format!(
            "{}/transfers/{}/completion",
            consumer_callback,
            consumer_pid.clone()
        );
        let req = self.client.post(consumer_url).json(&start_message).send().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            }
        })?;
        // process response
        let status = req.status();
        if status.clone().is_success() == false {
            bail!(DSRPCTransferProviderErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }
        // parse response
        let response = req.json::<TransferProcessMessage>().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            }
        })?;
        // persist transfer process
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
                    content: serde_json::to_value(start_message).unwrap(),
                },
            )
            .await
            .map_err(|e| {
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
            })?;
        self.data_plane_facade.on_transfer_completion().await?;
        let response = DSRPCTransferProviderCompletionResponse {
            provider_pid,
            consumer_pid,
            message: response,
        };
        self.notification_service.broadcast_notification(
            RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory: "TransferCompletionMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RPCMessage,
                message_operation: RainbowEventsNotificationMessageOperation::OutgoingMessage,
                message_content: json!({
                    "process": &process,
                    "message": &message
                }),
            }
        ).await?;
        Ok(response)
    }

    async fn setup_termination(
        &self,
        input: DSRPCTransferProviderTerminationRequest,
    ) -> anyhow::Result<DSRPCTransferProviderTerminationResponse> {
        let DSRPCTransferProviderTerminationRequest { consumer_callback, provider_pid, consumer_pid, code, reason, .. } =
            input;
        // validate fields
        let provider = self
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
        let consumer = self
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
        // validate correlation
        if provider.provider_pid != consumer.provider_pid {
            bail!(
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(
                    DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        provider_pid: Some(provider_pid.clone()),
                        consumer_pid: Some(consumer_pid.clone()),
                    }
                )
            );
        };
        // create message
        let start_message = TransferTerminationMessage {
            provider_pid: provider_pid.clone().to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            code: code.clone(),
            reason: reason.clone(),
            ..Default::default()
        };
        // http to consumer
        // TODO participants...
        let consumer_callback = consumer_callback.strip_suffix('/').unwrap_or(consumer_callback.as_str());
        let consumer_url = format!(
            "{}/transfers/{}/termination",
            consumer_callback,
            consumer_pid.clone()
        );
        let req = self.client.post(consumer_url).json(&start_message).send().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            }
        })?;
        // process response
        let status = req.status();
        if status.clone().is_success() == false {
            bail!(DSRPCTransferProviderErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }
        // parse response
        let response = req.json::<TransferProcessMessage>().await.map_err(|_e| {
            DSRPCTransferProviderErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            }
        })?;
        // persist transfer process
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
                    content: serde_json::to_value(start_message).unwrap(),
                },
            )
            .await
            .map_err(|e| {
                DSRPCTransferProviderErrors::DSProtocolTransferProviderError(DSProtocolTransferProviderErrors::DbErr(e))
            })?;
        self.data_plane_facade.on_transfer_termination().await?;
        let response = DSRPCTransferProviderTerminationResponse {
            provider_pid,
            consumer_pid,
            message: response,
        };
        self.notification_service.broadcast_notification(
            RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory: "TransferTerminationMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::RPCMessage,
                message_operation: RainbowEventsNotificationMessageOperation::OutgoingMessage,
                message_content: json!({
                    "process": &process,
                    "message": &message
                }),
            }
        ).await?;
        Ok(response)
    }
}
