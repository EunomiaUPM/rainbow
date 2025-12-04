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

use crate::consumer::core::data_plane_facade::DataPlaneConsumerFacadeTrait;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    DSRPCTransferConsumerCompletionRequest, DSRPCTransferConsumerCompletionResponse,
    DSRPCTransferConsumerRequestRequest, DSRPCTransferConsumerRequestResponse, DSRPCTransferConsumerStartRequest,
    DSRPCTransferConsumerStartResponse, DSRPCTransferConsumerSuspensionRequest,
    DSRPCTransferConsumerSuspensionResponse, DSRPCTransferConsumerTerminationRequest,
    DSRPCTransferConsumerTerminationResponse,
};
use crate::consumer::core::ds_protocol_rpc::DSRPCTransferConsumerTrait;
use anyhow::{anyhow, bail};
use axum::async_trait;
use rainbow_common::errors::helpers::{BadFormat, MissingAction};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::mates::Mates;
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_request::TransferRequestMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::protocol::transfer::{TransferMessageTypes, TransferRoles};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::transfer_consumer::entities::transfer_callback;
use rainbow_db::transfer_consumer::repo::{
    EditTransferCallback, NewTransferCallback, NewTransferMessageModel, TransferConsumerRepoFactory,
};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use reqwest::Client;
use serde_json::{json, to_value, Value};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error};
use urn::Urn;
use rainbow_common::config::services::TransferConfig;
use rainbow_common::config::traits::HostConfigTrait;
use rainbow_common::config::types::HostType;
use rainbow_common::facades::ssi_auth_facade::MatesFacadeTrait;

pub struct DSRPCTransferConsumerService<T, U, V, W>
where
    T: TransferConsumerRepoFactory + Send + Sync,
    U: DataPlaneConsumerFacadeTrait + Send + Sync,
    V: RainbowEventsNotificationTrait + Send + Sync + 'static,
    W: MatesFacadeTrait + Send + Sync,
{
    transfer_repo: Arc<T>,
    data_plane_facade: Arc<U>,
    config: TransferConfig,
    notification_service: Arc<V>,
    client: Client,
    mates_facade: Arc<W>,
}

impl<T, U, V, W> DSRPCTransferConsumerService<T, U, V, W>
where
    T: TransferConsumerRepoFactory + Send + Sync,
    U: DataPlaneConsumerFacadeTrait + Send + Sync,
    V: RainbowEventsNotificationTrait + Send + Sync + 'static,
    W: MatesFacadeTrait + Send + Sync,
{
    pub fn new(
        transfer_repo: Arc<T>,
        data_plane_facade: Arc<U>,
        config: TransferConfig,
        notification_service: Arc<V>,
        mates_facade: Arc<W>,
    ) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { transfer_repo, data_plane_facade, config, notification_service, client, mates_facade }
    }

    /// Get provider mate based in id
    async fn get_provider_mate(&self, provider_participant_id: &String) -> anyhow::Result<Mates> {
        debug!("DSProtocolRPC Service: get_provider_mate");

        let mate = self.mates_facade.get_mate_by_id(provider_participant_id.clone()).await.map_err(|e| {
            let e = CommonErrors::format_new(BadFormat::Received, &e.to_string());
            error!("{}", e.log());
            anyhow!(e)
        })?;
        Ok(mate)
    }

    /// Fetches and validates the existence of a transfer callback record by consumer_pid.
    async fn validate_and_get_transfer_callback_by_consumer_id(
        &self,
        provider_pid: &Urn,
        consumer_pid: &Urn,
    ) -> anyhow::Result<transfer_callback::Model> {
        debug!("DSProtocolRPC Service: validate_and_get_transfer_callback_by_consumer_id");

        let consumer_process = self
            .transfer_repo
            .get_transfer_callback_by_consumer_id(consumer_pid.clone())
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
        let provider_process = self
            .transfer_repo
            .get_transfer_callback_by_provider_id(provider_pid.clone())
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
        if consumer_process.consumer_pid != provider_process.consumer_pid {
            let e = CommonErrors::format_new(
                BadFormat::Received,
                "ConsumerPid and ProviderPid don't coincide",
            );
            error!("{}", e.log());
            bail!(e);
        }
        Ok(consumer_process.into())
    }

    // Helper function to send protocol messages to the provider
    async fn send_protocol_message_to_provider<M: serde::Serialize + std::fmt::Debug>(
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
            let e = CommonErrors::provider_new(
                &target_url,
                "POST",
                None,
                &format!("Provider Internal error: {}", consumer_error),
            );
            error!("{}", e.log());
            bail!(e);
        }

        let process_message = response.json::<TransferProcessMessage>().await.map_err(|_e| {
            let e = CommonErrors::format_new(
                BadFormat::Received,
                "TransferProcessMessage not serializable",
            );
            error!("{}", e.log());
            anyhow!(e)
        })?;
        Ok(process_message)
    }

    /// Broadcasts a notification about a transfer process event.
    async fn notify_subscribers(&self, subcategory: String, message: serde_json::Value) -> anyhow::Result<()> {
        debug!("DSProtocolRPC Service: notify_subscribers");

        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess, // Specific category for transfers
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
impl<T, U, V, W> DSRPCTransferConsumerTrait for DSRPCTransferConsumerService<T, U, V, W>
where
    T: TransferConsumerRepoFactory + Send + Sync,
    U: DataPlaneConsumerFacadeTrait + Send + Sync,
    V: RainbowEventsNotificationTrait + Send + Sync + 'static,
    W: MatesFacadeTrait + Send + Sync,
{
    async fn setup_request(
        &self,
        input: DSRPCTransferConsumerRequestRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerRequestResponse> {
        let DSRPCTransferConsumerRequestRequest { agreement_id, format, data_address, provider_participant_id, .. } =
            input.clone();
        // 0. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Unknown, "No base url");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Token, "No auth token");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        // 1. Generate PIDs and callback address
        let consumer_pid = get_urn(None);
        let callback_urn = get_urn(None);
        let callback_address = format!(
            "{}/{}",
            self.config.get_host(HostType::Http),
            callback_urn
        );
        // 2. Create message
        let transfer_request = TransferRequestMessage {
            consumer_pid: consumer_pid.clone(),
            agreement_id: agreement_id.clone(),
            format: format.clone(),
            data_address: data_address.clone(),
            callback_address: Some(callback_address.to_string()),
            ..Default::default()
        };
        // 3. Send message to provider
        let provider_url = format!("{}/transfers/request", provider_base_url);
        let response = self
            .send_protocol_message_to_provider(
                provider_url,
                &transfer_request,
                provider_token,
                None, // Provider PID is not known yet for error tracking at this initial stage
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist in DB (create transfer callback)
        let transfer_process = self
            .transfer_repo
            .create_transfer_callback(NewTransferCallback {
                callback_id: Some(callback_urn),
                consumer_pid: Some(consumer_pid.clone()),
                provider_pid: Some(get_urn_from_string(&response.provider_pid)?),
                data_address: None,
                associated_provider: Some(provider_mate.participant_id),
            })
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        let message = self
            .transfer_repo
            .create_transfer_message(
                get_urn_from_string(&transfer_process.id)?,
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferRequestMessage.to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: to_value(&input)?,
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // 5. Data plane facade hook
        self.data_plane_facade.on_transfer_request(consumer_pid.clone(), format.clone()).await?;
        // 6. Create response
        let provider_pid = Some(get_urn_from_string(
            &transfer_process.provider_pid.clone().unwrap(),
        )?);
        let response = DSRPCTransferConsumerRequestResponse {
            provider_pid: provider_pid.unwrap(),
            consumer_pid,
            agreement_id,
            format,
            data_address,
            callback_address: callback_address.to_string(),
            message: response.clone(),
        };
        // 7. Notify subscribers
        self.notify_subscribers(
            "TransferRequestMessage".to_string(),
            json!({
                "process": transfer_process,
                "message": message,
            }),
        )
        .await?;
        // 8. Bye
        Ok(response)
    }

    async fn setup_start(
        &self,
        input: DSRPCTransferConsumerStartRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerStartResponse> {
        let DSRPCTransferConsumerStartRequest {
            data_address, provider_participant_id, provider_pid, consumer_pid, ..
        } = input.clone();
        // 0. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Unknown, "No base url");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Token, "No auth token");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        // 1. Validate correlation
        let _current_process_record =
            self.validate_and_get_transfer_callback_by_consumer_id(&provider_pid, &consumer_pid).await?;
        // 2. Create message
        let transfer_start_message = TransferStartMessage {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            data_address: data_address.clone(),
            ..Default::default()
        };
        // 3. Send message to provider
        let provider_url = format!(
            "{}/transfers/{}/start",
            provider_base_url,
            provider_pid.to_string()
        );
        let response = self
            .send_protocol_message_to_provider(
                provider_url,
                &transfer_start_message,
                provider_token,
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist in DB (update transfer callback)
        let transfer_process = self
            .transfer_repo
            .put_transfer_callback_by_consumer(
                consumer_pid.clone(),
                EditTransferCallback {
                    consumer_pid: None,
                    provider_pid: None,
                    data_plane_id: None,
                    data_address: None,
                    restart_flag: Some(true),
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        let message = self
            .transfer_repo
            .create_transfer_message(
                get_urn_from_string(&transfer_process.id)?,
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferStartMessage.to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: to_value(&input)?,
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // 5. Data plane facade hook
        if transfer_process.restart_flag {
            self.data_plane_facade.on_transfer_restart(consumer_pid.clone()).await?;
        } else {
            self.data_plane_facade.on_transfer_start(consumer_pid.clone(), data_address.clone()).await?;
        }
        // 6. Create response
        let response = DSRPCTransferConsumerStartResponse {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            data_address,
            message: response,
        };
        // 7. Notify subscribers
        self.notify_subscribers(
            "TransferStartMessage".to_string(),
            json!({
                "process": transfer_process,
                "message": message,
            }),
        )
        .await?;
        // 8. Bye
        Ok(response)
    }

    async fn setup_suspension(
        &self,
        input: DSRPCTransferConsumerSuspensionRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerSuspensionResponse> {
        let DSRPCTransferConsumerSuspensionRequest {
            provider_participant_id,
            provider_pid,
            consumer_pid,
            code,
            reason,
        } = input.clone();
        // 0. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Unknown, "No base url");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Token, "No auth token");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        // 1. Validate correlation
        let _current_process_record =
            self.validate_and_get_transfer_callback_by_consumer_id(&provider_pid, &consumer_pid).await?;
        // 2. Create message
        let transfer_suspension_message = TransferSuspensionMessage {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            code,
            reason,
            ..Default::default()
        };
        // 3. Send message to provider
        let provider_url = format!(
            "{}/transfers/{}/suspension",
            provider_base_url,
            provider_pid.to_string()
        );
        let response = self
            .send_protocol_message_to_provider(
                provider_url,
                &transfer_suspension_message,
                provider_token,
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist in DB (update transfer callback)
        let transfer_process = self
            .transfer_repo
            .put_transfer_callback_by_consumer(
                consumer_pid.clone(),
                EditTransferCallback {
                    consumer_pid: None,
                    provider_pid: None,
                    data_plane_id: None,
                    data_address: None,
                    restart_flag: Some(true),
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        let message = self
            .transfer_repo
            .create_transfer_message(
                get_urn_from_string(&transfer_process.id)?,
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: to_value(&input)?,
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // 5. Data plane facade hook
        self.data_plane_facade.on_transfer_suspension(consumer_pid.clone()).await?;
        // 6. Create response
        let response = DSRPCTransferConsumerSuspensionResponse {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            message: response,
        };
        // 7. Notify subscribers
        self.notify_subscribers(
            "TransferSuspensionMessage".to_string(),
            json!({
                "process": transfer_process,
                "message": message,
            }),
        )
        .await?;
        // 8. Bye
        Ok(response)
    }

    async fn setup_completion(
        &self,
        input: DSRPCTransferConsumerCompletionRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerCompletionResponse> {
        let DSRPCTransferConsumerCompletionRequest { provider_participant_id, provider_pid, consumer_pid, .. } =
            input.clone();
        // 0. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Unknown, "No base url");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Token, "No auth token");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        // 1. Validate correlation
        let _current_process_record =
            self.validate_and_get_transfer_callback_by_consumer_id(&provider_pid, &consumer_pid).await?;
        // 2. Create message
        let transfer_completion_message = TransferCompletionMessage {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            ..Default::default()
        };
        // 3. Send message to provider
        let provider_address = format!(
            "{}/transfers/{}/completion",
            provider_base_url,
            provider_pid.to_string()
        );
        let response = self
            .send_protocol_message_to_provider(
                provider_address,
                &transfer_completion_message,
                provider_token,
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist in DB (update transfer callback)
        let transfer_process = self
            .transfer_repo
            .put_transfer_callback_by_consumer(
                consumer_pid.clone(),
                EditTransferCallback {
                    consumer_pid: None,
                    provider_pid: None,
                    data_plane_id: None,
                    data_address: None,
                    restart_flag: Some(false),
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        let message = self
            .transfer_repo
            .create_transfer_message(
                get_urn_from_string(&transfer_process.id)?,
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferCompletionMessage.to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: to_value(&input)?,
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // 5. Data plane facade hook
        self.data_plane_facade.on_transfer_completion(consumer_pid.clone()).await?;
        // 6. Create response
        let response = DSRPCTransferConsumerCompletionResponse {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            message: response,
        };
        // 7. Notify subscribers
        self.notify_subscribers(
            "TransferCompletionMessage".to_string(),
            json!({
                "process": transfer_process,
                "message": message,
            }),
        )
        .await?;
        // 8. Bye
        Ok(response)
    }

    async fn setup_termination(
        &self,
        input: DSRPCTransferConsumerTerminationRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerTerminationResponse> {
        let DSRPCTransferConsumerTerminationRequest {
            provider_participant_id,
            provider_pid,
            consumer_pid,
            code,
            reason,
        } = input.clone();
        // 0. fetch participant
        let provider_mate = self.get_provider_mate(&provider_participant_id).await?;
        let provider_base_url = provider_mate.base_url.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Unknown, "No base url");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        let provider_base_url = provider_base_url.strip_suffix('/').unwrap_or(provider_base_url.as_str());
        let provider_token = provider_mate.token.ok_or_else(|| {
            let e = CommonErrors::missing_action_new(MissingAction::Token, "No auth token");
            error!("{}", e.log());
            anyhow!(e)
        })?;
        // 1. Validate correlation
        let _current_process_record =
            self.validate_and_get_transfer_callback_by_consumer_id(&provider_pid, &consumer_pid).await?;
        // 2. Create message
        let transfer_termination_message = TransferTerminationMessage {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            code,
            reason,
            ..Default::default()
        };
        // 3. Send message to provider
        let provider_url = format!(
            "{}/transfers/{}/termination",
            provider_base_url,
            provider_pid.to_string()
        );
        let response = self
            .send_protocol_message_to_provider(
                provider_url,
                &transfer_termination_message,
                provider_token,
                Some(provider_pid.clone()),
                Some(consumer_pid.clone()),
            )
            .await?;
        // 4. Persist in DB (update transfer callback)
        let transfer_process = self
            .transfer_repo
            .put_transfer_callback_by_consumer(
                consumer_pid.clone(),
                EditTransferCallback {
                    consumer_pid: None,
                    provider_pid: None,
                    data_plane_id: None,
                    data_address: None,
                    restart_flag: Some(false),
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        let message = self
            .transfer_repo
            .create_transfer_message(
                get_urn_from_string(&transfer_process.id)?,
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferTerminationMessage.to_string(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: to_value(&input)?,
                },
            )
            .await
            .map_err(|e| {
                let e_ = CommonErrors::database_new(&e.to_string());
                error!("{}", e_.log());
                anyhow!(e_)
            })?;
        // 5. Data plane facade hook
        self.data_plane_facade.on_transfer_termination(consumer_pid.clone()).await?;
        // 6. Create response
        let response = DSRPCTransferConsumerTerminationResponse {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            message: response,
        };
        // 7. Notify subscribers
        self.notify_subscribers(
            "TransferTerminationMessage".to_string(),
            json!({
                "process": transfer_process,
                "message": message,
            }),
        )
        .await?;
        // 8. Bye
        Ok(response)
    }
}
