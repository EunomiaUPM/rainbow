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
use crate::consumer::core::data_plane_facade::DataPlaneConsumerFacadeTrait;
use crate::consumer::core::ds_protocol::ds_protocol_err::DSProtocolTransferConsumerErrors;
use crate::consumer::core::ds_protocol::DSProtocolTransferConsumerTrait;
use crate::consumer::core::rainbow_entities::rainbow_err::RainbowTransferConsumerErrors;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::facades::ssi_auth_facade::SSIAuthFacadeTrait;
use rainbow_common::mates::Mates;
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_protocol_trait::DSProtocolTransferMessageTrait;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::protocol::transfer::{TransferMessageTypes, TransferRoles, TransferState};
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::transfer_consumer::entities::transfer_callback;
use rainbow_db::transfer_consumer::repo::{EditTransferCallback, NewTransferMessageModel, TransferConsumerRepoFactory};
use rainbow_events::core::notification::notification_types::{
    RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory,
    RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes,
};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use tracing::debug;
use urn::Urn;

pub struct DSProtocolTransferConsumerService<T, U, V, W>
where
    T: TransferConsumerRepoFactory + Send + Sync + 'static,
    U: DataPlaneConsumerFacadeTrait + Send + Sync + 'static,
    V: RainbowEventsNotificationTrait + Send + Sync + 'static, // Added notification service
    W: SSIAuthFacadeTrait + Sync + Send,
{
    transfer_repo: Arc<T>,
    data_plane: Arc<U>,
    notification_service: Arc<V>, // Added notification service
    ssi_auth_facade: Arc<W>,
}

impl<T, U, V, W> DSProtocolTransferConsumerService<T, U, V, W>
where
    T: TransferConsumerRepoFactory + Send + Sync + 'static,
    U: DataPlaneConsumerFacadeTrait + Send + Sync + 'static,
    V: RainbowEventsNotificationTrait + Send + Sync + 'static,
    W: SSIAuthFacadeTrait + Sync + Send,
{
    pub fn new(
        transfer_repo: Arc<T>,
        data_plane: Arc<U>,
        notification_service: Arc<V>,
        ssi_auth_facade: Arc<W>,
    ) -> Self {
        Self { transfer_repo, data_plane, notification_service, ssi_auth_facade }
    }

    /// Validate auth token
    async fn validate_auth_token(&self, token: String) -> anyhow::Result<Mates> {
        let mate = self.ssi_auth_facade.verify_token(token).await?;
        Ok(mate)
    }

    /// Performs JSON schema validation for incoming messages.
    fn json_schema_validation<'a, M: DSProtocolTransferMessageTrait<'a>>(&self, message: &M) -> anyhow::Result<()> {
        debug!("Transfer consumer JSON schema validation");
        validate_payload_schema(message)?;
        Ok(())
    }

    /// Validates the payload of an incoming transfer message against existing transfer processes.
    /// Ensures correlation between URIs and message body PIDs.
    async fn payload_validation<'a, M: DSProtocolTransferMessageTrait<'a>>(
        &self,
        callback_id: &Option<Urn>,   // Optional callback_id from URL
        consumer_pid_from_uri: &Urn, // Consumer PID from URL path
        message: &M,                 // The incoming message
        provider_participant_mate: &Mates,
    ) -> anyhow::Result<transfer_callback::Model> {
        debug!("Transfer consumer payload validation");
        let message_consumer_pid = message.get_consumer_pid()?;
        let message_provider_pid = message.get_provider_pid()?;

        // 1. Validate consumer_pid in message body matches consumer_pid from URI
        match message_consumer_pid {
            Some(msg_c_pid) if msg_c_pid == consumer_pid_from_uri => {}
            _ => bail!(DSProtocolTransferConsumerErrors::UriAndBodyIdentifiersDoNotCoincide),
        }
        // 2. Validate correlation in processes
        let consumer_transfer_process = self
            .transfer_repo
            .get_transfer_callback_by_consumer_id(consumer_pid_from_uri.clone())
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?
            .ok_or(DSProtocolTransferConsumerErrors::TransferProcessNotFound {
                provider_pid: message_provider_pid.map(|m| m.to_owned()),
                consumer_pid: Some(consumer_pid_from_uri.to_owned()),
            })?;
        if let Some(provider_pid) = message_provider_pid.clone() {
            let provider_transfer_process = self
                .transfer_repo
                .get_transfer_callback_by_provider_id(provider_pid.to_owned())
                .await
                .map_err(DSProtocolTransferConsumerErrors::DbErr)?
                .ok_or(DSProtocolTransferConsumerErrors::TransferProcessNotFound {
                    provider_pid: message_provider_pid.map(|m| m.to_owned()),
                    consumer_pid: Some(consumer_pid_from_uri.to_owned()),
                })?;
            if consumer_transfer_process.consumer_pid != provider_transfer_process.consumer_pid {
                bail!(RainbowTransferConsumerErrors::ValidationError(
                    "ConsumerPid and ProviderPid don't coincide".to_string()
                ))
            }
        }
        // 3. Validate provider_pid in message body matches provider_pid in DB
        match (
            message_provider_pid,
            consumer_transfer_process.provider_pid.as_ref(),
        ) {
            (Some(msg_p_pid), Some(db_p_pid)) if msg_p_pid.to_string() == db_p_pid.to_owned() => {}
            _ => bail!(DSProtocolTransferConsumerErrors::UriAndBodyIdentifiersDoNotCoincide),
        }
        // 4. Consumer transfer process and provider participant mate
        if consumer_transfer_process.associated_provider.clone().unwrap() != provider_participant_mate.participant_id {
            bail!(RainbowTransferConsumerErrors::ValidationError(
                "This user is not related with this process".to_string()
            ))
        }

        Ok(consumer_transfer_process.into())
    }

    /// Broadcasts a notification about a transfer process event.
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
impl<T, U, V, W> DSProtocolTransferConsumerTrait for DSProtocolTransferConsumerService<T, U, V, W>
where
    T: TransferConsumerRepoFactory + Send + Sync + 'static,
    U: DataPlaneConsumerFacadeTrait + Send + Sync + 'static,
    V: RainbowEventsNotificationTrait + Send + Sync + 'static,
    W: SSIAuthFacadeTrait + Sync + Send,
{
    async fn get_transfer_requests_by_callback(&self, callback_id: Urn) -> anyhow::Result<TransferProcessMessage> {
        let transfer_process = self
            .transfer_repo
            .get_transfer_callbacks_by_id(callback_id)
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?
            .ok_or(DSProtocolTransferConsumerErrors::TransferProcessNotFound {
                provider_pid: None,
                consumer_pid: None,
            })?;
        Ok(transfer_process.into())
    }
    async fn get_transfer_requests_by_provider(&self, _provider_pid: Urn) -> anyhow::Result<TransferProcessMessage> {
        todo!()
    }

    async fn get_transfer_requests_by_consumer(&self, consumer_pid: Urn) -> anyhow::Result<TransferProcessMessage> {
        let transfer_process = self
            .transfer_repo
            .get_transfer_callback_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?
            .ok_or(DSProtocolTransferConsumerErrors::TransferProcessNotFound {
                provider_pid: None,
                consumer_pid: Some(consumer_pid),
            })?;
        Ok(transfer_process.into())
    }

    async fn transfer_start(
        &self,
        callback_id: Option<Urn>,
        consumer_pid: Urn,
        input: TransferStartMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferStartMessage { provider_pid, consumer_pid: consumer_pid_, data_address, .. } = input.clone();
        // 1. Validate request
        let provider_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input)
            .map_err(|e| RainbowTransferConsumerErrors::ValidationError(e.to_string()))?;
        let _existing_process = self.payload_validation(&callback_id, &consumer_pid, &input, &provider_participant_mate).await?;
        // 2. Persist model
        let callback = self
            .transfer_repo
            .put_transfer_callback(
                callback_id.clone().unwrap(),
                EditTransferCallback {
                    provider_pid: Option::from(provider_pid),
                    data_address: Option::from(to_value(data_address.clone())?),
                    ..Default::default()
                },
            )
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?;
        let message = self
            .transfer_repo
            .create_transfer_message(
                callback_id.clone().unwrap(),
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferStartMessage.to_string(),
                    from: TransferRoles::Provider,
                    to: TransferRoles::Consumer,
                    content: to_value(&input)?,
                },
            )
            .await?;
        // 3. Data plane hook
        if callback.restart_flag {
            self.data_plane.on_transfer_restart(consumer_pid.clone()).await?;
        } else {
            self.data_plane.on_transfer_start(consumer_pid.clone(), data_address.clone()).await?;
        }
        // 4. Prepare response
        let mut transfer_process: TransferProcessMessage = callback.into();
        transfer_process.state = TransferState::STARTED;
        // 5. Notify subscribers
        self.notify_subscribers(
            "TransferStartMessage".to_string(),
            json!({
                "transfer_process": transfer_process,
                "transfer_message": message,
            }),
        )
            .await?;
        // 6. Bye
        Ok(transfer_process)
    }

    async fn transfer_suspension(
        &self,
        callback_id: Option<Urn>,
        consumer_pid: Urn,
        input: TransferSuspensionMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferSuspensionMessage { provider_pid, consumer_pid: consumer_pid_, .. } = input.clone();
        // 1. Validate request
        let provider_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input)
            .map_err(|e| RainbowTransferConsumerErrors::ValidationError(e.to_string()))?;
        let _existing_process = self.payload_validation(&callback_id, &consumer_pid, &input, &provider_participant_mate).await?;
        // 2. Persist state
        let callback = self
            .transfer_repo
            .put_transfer_callback(
                callback_id.clone().unwrap(),
                EditTransferCallback { ..Default::default() },
            )
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?;
        let message = self
            .transfer_repo
            .create_transfer_message(
                callback_id.clone().unwrap(),
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
                    from: TransferRoles::Provider,
                    to: TransferRoles::Consumer,
                    content: to_value(&input)?,
                },
            )
            .await?;
        // 3. Data plane hook
        self.data_plane.on_transfer_suspension(consumer_pid.clone()).await?;
        // 4. Prepare response
        let mut transfer_process: TransferProcessMessage = callback.into();
        transfer_process.state = TransferState::SUSPENDED;
        // 5. Notify subscribers
        self.notify_subscribers(
            "TransferSuspensionMessage".to_string(),
            json!({
                "transfer_process": transfer_process,
                "transfer_message": message,
            }),
        )
            .await?;
        // 6. Bye
        Ok(transfer_process)
    }

    async fn transfer_completion(
        &self,
        callback_id: Option<Urn>,
        consumer_pid: Urn,
        input: TransferCompletionMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferCompletionMessage { provider_pid, consumer_pid: consumer_pid_, .. } = input.clone();
        // 1. Validate request
        let provider_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input)
            .map_err(|e| RainbowTransferConsumerErrors::ValidationError(e.to_string()))?;
        let _existing_process = self.payload_validation(&callback_id, &consumer_pid, &input, &provider_participant_mate).await?;
        // 2. Persist state
        let callback = self
            .transfer_repo
            .put_transfer_callback(
                callback_id.clone().unwrap(),
                EditTransferCallback { ..Default::default() },
            )
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?;
        let message = self
            .transfer_repo
            .create_transfer_message(
                callback_id.clone().unwrap(),
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
                    from: TransferRoles::Provider,
                    to: TransferRoles::Consumer,
                    content: to_value(&input)?,
                },
            )
            .await?;
        // 3. Data plane hook
        self.data_plane.on_transfer_completion(consumer_pid.clone()).await?;
        // 4. Prepare response
        let mut transfer_process: TransferProcessMessage = callback.into();
        transfer_process.state = TransferState::COMPLETED;
        // 5. Notify subscribers
        self.notify_subscribers(
            "TransferCompletionMessage".to_string(),
            json!({
                "transfer_process": transfer_process,
                "transfer_message": message,

            }),
        )
            .await?;
        // 6. Bye
        Ok(transfer_process)
    }

    async fn transfer_termination(
        &self,
        callback_id: Option<Urn>,
        consumer_pid: Urn,
        input: TransferTerminationMessage,
        token: String,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferTerminationMessage { provider_pid, consumer_pid: consumer_pid_, .. } = input.clone();
        // 1. Validate request
        let provider_participant_mate = self.validate_auth_token(token).await?;
        self.json_schema_validation(&input)
            .map_err(|e| RainbowTransferConsumerErrors::ValidationError(e.to_string()))?;
        let _existing_process = self.payload_validation(&callback_id, &consumer_pid, &input, &provider_participant_mate).await?;
        // 2. Persist state
        let callback = self
            .transfer_repo
            .put_transfer_callback(
                callback_id.clone().unwrap(),
                EditTransferCallback { ..Default::default() },
            )
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?;
        let message = self
            .transfer_repo
            .create_transfer_message(
                callback_id.clone().unwrap(),
                NewTransferMessageModel {
                    message_type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
                    from: TransferRoles::Provider,
                    to: TransferRoles::Consumer,
                    content: to_value(&input)?,
                },
            )
            .await?;
        // 3. Data plane hook
        self.data_plane.on_transfer_termination(consumer_pid.clone()).await?;
        // 4. Prepare response
        let mut transfer_process: TransferProcessMessage = callback.into();
        transfer_process.state = TransferState::TERMINATED;
        // 5. Notify subscribers
        self.notify_subscribers(
            "TransferTerminationMessage".to_string(),
            json!({
                "transfer_process": transfer_process,
                "transfer_message": message,

            }),
        )
            .await?;
        // 6. Bye
        Ok(transfer_process)
    }
}
