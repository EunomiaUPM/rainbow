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

use crate::common::utils::has_data_address_in_push;
use crate::provider::core::data_plane_facade::DataPlaneProviderFacadeTrait;
use crate::provider::core::data_service_resolver_facade::DataServiceFacadeTrait;
use crate::provider::core::ds_protocol::ds_protocol_err::DSProtocolTransferProviderErrors;
use crate::provider::core::ds_protocol::DSProtocolTransferProviderTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_request::TransferRequestMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::protocol::transfer::{TransferRoles, TransferState, TransferStateAttribute};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_dataplane::coordinator::controller::DataPlaneControllerTrait;
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

pub struct DSProtocolTransferProviderImpl<T, U, V, W>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneProviderFacadeTrait + Send + Sync,
    W: RainbowEventsNotificationTrait + Sync + Send,
{
    transfer_repo: Arc<T>,
    data_service_facade: Arc<U>,
    data_plane: Arc<V>,
    notification_service: Arc<W>,
}

impl<T, U, V, W> DSProtocolTransferProviderImpl<T, U, V, W>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneProviderFacadeTrait + Send + Sync,
    W: RainbowEventsNotificationTrait + Sync + Send,
{
    pub fn new(
        transfer_repo: Arc<T>,
        data_service_facade: Arc<U>,
        data_plane: Arc<V>,
        notification_service: Arc<W>,
    ) -> Self {
        Self { transfer_repo, data_service_facade, data_plane, notification_service }
    }
}

#[async_trait]
impl<T, U, V, W> DSProtocolTransferProviderTrait for DSProtocolTransferProviderImpl<T, U, V, W>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneProviderFacadeTrait + Send + Sync,
    W: RainbowEventsNotificationTrait + Sync + Send,
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

    async fn transfer_request(&self, input: TransferRequestMessage) -> anyhow::Result<TransferProcessMessage> {
        debug!("DSProtocol Service: transfer_request");
        // extract data
        let provider_pid = get_urn(None);
        let consumer_pid = input.consumer_pid.to_owned();
        let agreement_id = get_urn_from_string(&input.agreement_id)?;
        let formats = input.format.clone();
        let _created_at = chrono::Utc::now().naive_utc();
        let message_type = input._type.clone();

        // validate
        if has_data_address_in_push(&input.data_address, &input.format)? == false {
            bail!(
                DSProtocolTransferProviderErrors::DataAddressCannotBeNullOnPushError {
                    consumer_pid: Option::from(consumer_pid),
                    provider_pid: None
                }
            );
        }

        // resolve data service
        let data_service =
            self.data_service_facade.resolve_data_service_by_agreement_id(agreement_id.clone(), Some(formats)).await?;
        // connect to data plane
        self.data_plane.on_transfer_request(provider_pid.clone(), data_service, input.format.clone()).await?;

        // db persist
        let transfer_process = self
            .transfer_repo
            .create_transfer_process(NewTransferProcessModel {
                provider_pid: provider_pid.clone(),
                consumer_pid,
                agreement_id,
                // data_plane_id,
                data_plane_id: get_urn(None), // TODO
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

        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory: "TransferRequestMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
                message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
                message_content: json!({
                    "process": transfer_process.clone(),
                    "message": transfer_message
                }),
            })
            .await?;
        Ok(transfer_process.into())
    }

    async fn transfer_start(
        &self,
        provider_pid: Urn,
        input: TransferStartMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferStartMessage { provider_pid: provider_pid_, consumer_pid, .. } = input.clone();

        // validate
        if provider_pid != provider_pid_ {
            bail!(DSProtocolTransferProviderErrors::UriAndBodyIdentifiersDoNotCoincide);
        }

        //
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

        // data plane
        let _data_plane_id = get_urn_from_string(&transfer_process.data_plane_id.clone().unwrap())?;
        // self.data_plane.connect_to_streaming_service(data_plane_id).await?;
        self.data_plane.on_transfer_start(provider_pid.clone()).await?;

        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory: "TransferStartMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
                message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
                message_content: json!({
                    "process": transfer_process.clone(),
                    "message": transfer_message
                }),
            })
            .await?;
        Ok(transfer_process.into())
    }

    async fn transfer_suspension(
        &self,
        provider_pid: Urn,
        input: TransferSuspensionMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferSuspensionMessage { provider_pid: provider_pid_, consumer_pid, .. } = input.clone();

        // validate
        if provider_pid != provider_pid_ {
            bail!(DSProtocolTransferProviderErrors::UriAndBodyIdentifiersDoNotCoincide);
        }

        // persist process
        let transfer_process_db = self
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

        // data plane
        let _data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.clone().unwrap())?;
        // self.data_plane.disconnect_from_streaming_service(data_plane_id).await?;
        self.data_plane.on_transfer_suspension(provider_pid.clone()).await?;

        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory: "TransferSuspensionMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
                message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
                message_content: json!({
                    "process": transfer_process_db.clone(),
                    "message": transfer_message
                }),
            })
            .await?;
        Ok(transfer_process_db.into())
    }

    async fn transfer_completion(
        &self,
        provider_pid: Urn,
        input: TransferCompletionMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferCompletionMessage { provider_pid: provider_pid_, consumer_pid, .. } = input.clone();

        // validate
        if provider_pid != provider_pid_ {
            bail!(DSProtocolTransferProviderErrors::UriAndBodyIdentifiersDoNotCoincide);
        }

        // persist process
        let transfer_process_db = self
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

        // data plane
        let _data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.clone().unwrap())?;
        // self.data_plane.disconnect_from_streaming_service(data_plane_id).await?;
        self.data_plane.on_transfer_completion(provider_pid.clone()).await?;

        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory: "TransferCompletionMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
                message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
                message_content: json!({
                    "process": transfer_process_db.clone(),
                    "message": transfer_message
                }),
            })
            .await?;
        Ok(transfer_process_db.into())
    }

    async fn transfer_termination(
        &self,
        provider_pid: Urn,
        input: TransferTerminationMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferTerminationMessage { provider_pid: provider_pid_, consumer_pid, .. } = input.clone();

        // validate
        if provider_pid != provider_pid_ {
            bail!(DSProtocolTransferProviderErrors::UriAndBodyIdentifiersDoNotCoincide);
        }

        // persist process
        let transfer_process_db = self
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

        // data plane
        let _data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.clone().unwrap())?;
        // self.data_plane.disconnect_from_streaming_service(data_plane_id).await?;
        self.data_plane.on_transfer_termination(provider_pid.clone()).await?;

        self.notification_service
            .broadcast_notification(RainbowEventsNotificationBroadcastRequest {
                category: RainbowEventsNotificationMessageCategory::TransferProcess,
                subcategory: "TransferTerminationMessage".to_string(),
                message_type: RainbowEventsNotificationMessageTypes::DSProtocolMessage,
                message_operation: RainbowEventsNotificationMessageOperation::IncomingMessage,
                message_content: json!({
                    "process": transfer_process_db.clone(),
                    "message": transfer_message
                }),
            })
            .await?;
        Ok(transfer_process_db.into())
    }
}
