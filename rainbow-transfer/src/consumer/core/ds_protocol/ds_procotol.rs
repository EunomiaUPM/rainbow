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

use crate::consumer::core::data_plane_facade::DataPlaneConsumerFacadeTrait;
use crate::consumer::core::ds_protocol::ds_protocol_err::DSProtocolTransferConsumerErrors;
use crate::consumer::core::ds_protocol::DSProtocolTransferConsumerTrait;
use axum::async_trait;
use rainbow_common::protocol::transfer::transfer_completion::TransferCompletionMessage;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::protocol::transfer::transfer_termination::TransferTerminationMessage;
use rainbow_common::protocol::transfer::TransferState;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::transfer_consumer::repo::{EditTransferCallback, TransferConsumerRepoFactory};
use serde_json::to_value;
use std::sync::Arc;
use urn::Urn;

pub struct DSProtocolTransferConsumerService<T, U>
where
    T: TransferConsumerRepoFactory + Send + Sync,
    U: DataPlaneConsumerFacadeTrait + Send + Sync,
{
    transfer_repo: Arc<T>,
    data_plane: Arc<U>,
}

impl<T, U> DSProtocolTransferConsumerService<T, U>
where
    T: TransferConsumerRepoFactory + Send + Sync,
    U: DataPlaneConsumerFacadeTrait + Send + Sync,
{
    pub fn new(transfer_repo: Arc<T>, data_plane: Arc<U>) -> Self {
        Self { transfer_repo, data_plane }
    }
    async fn do_validations(&self,
                            callback_id: &Option<Urn>,
                            consumer_pid: &Urn,
                            input_consumer_pid: &String,
                            input_provider_pid: &String,
    ) -> anyhow::Result<()> {
        // validate consumer
        if &consumer_pid.to_string() != input_consumer_pid {
            return Err(DSProtocolTransferConsumerErrors::UriAndBodyIdentifiersDoNotCoincide.into());
        }
        let consumer = self
            .transfer_repo
            .get_transfer_callback_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?
            .ok_or(DSProtocolTransferConsumerErrors::TransferProcessNotFound {
                provider_pid: None,
                consumer_pid: Some(consumer_pid.clone()),
            })?;
        // validate callback
        if let Some(callback_id) = callback_id.clone() {
            let callback = self
                .transfer_repo
                .get_transfer_callbacks_by_id(callback_id.clone())
                .await
                .map_err(DSProtocolTransferConsumerErrors::DbErr)?
                .ok_or(DSProtocolTransferConsumerErrors::TransferProcessNotFound {
                    provider_pid: None,
                    consumer_pid: None,
                })?;
            if callback.consumer_pid != consumer.consumer_pid {
                return Err(DSProtocolTransferConsumerErrors::UriAndBodyIdentifiersDoNotCoincide.into());
            }
        }
        if &consumer.provider_pid.unwrap() != input_provider_pid {
            return Err(DSProtocolTransferConsumerErrors::UriAndBodyIdentifiersDoNotCoincide.into());
        }
        Ok(())
    }
}

#[async_trait]
impl<T, U> DSProtocolTransferConsumerTrait for DSProtocolTransferConsumerService<T, U>
where
    T: TransferConsumerRepoFactory + Send + Sync,
    U: DataPlaneConsumerFacadeTrait + Send + Sync,
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
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferStartMessage { provider_pid, consumer_pid: consumer_pid_, data_address, .. } = input;

        self.do_validations(
            &callback_id,
            &consumer_pid,
            &consumer_pid_,
            &provider_pid,
        ).await?;

        // persist model
        let callback = self
            .transfer_repo
            .put_transfer_callback(
                callback_id.unwrap(),
                EditTransferCallback {
                    provider_pid: Option::from(get_urn_from_string(&provider_pid)?),
                    data_address: Option::from(to_value(data_address.clone())?),
                    ..Default::default()
                },
            )
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?;

        // data plane
        self.data_plane.on_transfer_start(consumer_pid.clone(), data_address.clone()).await?;

        // return
        let mut transfer_process: TransferProcessMessage = callback.into();
        transfer_process.state = TransferState::STARTED;
        Ok(transfer_process)
    }

    async fn transfer_suspension(
        &self,
        callback_id: Option<Urn>,
        consumer_pid: Urn,
        input: TransferSuspensionMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferSuspensionMessage { provider_pid, consumer_pid: consumer_pid_, .. } = input;

        self.do_validations(
            &callback_id,
            &consumer_pid,
            &consumer_pid_,
            &provider_pid,
        ).await?;

        // persist model
        let callback = self
            .transfer_repo
            .put_transfer_callback(
                callback_id.unwrap(),
                EditTransferCallback {
                    ..Default::default()
                },
            )
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?;

        // data plane
        self.data_plane.on_transfer_suspension(consumer_pid.clone()).await?;

        // return
        let mut transfer_process: TransferProcessMessage = callback.into();
        transfer_process.state = TransferState::SUSPENDED;
        Ok(transfer_process)
    }

    async fn transfer_completion(
        &self,
        callback_id: Option<Urn>,
        consumer_pid: Urn,
        input: TransferCompletionMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferCompletionMessage { provider_pid, consumer_pid: consumer_pid_, .. } = input;

        self.do_validations(
            &callback_id,
            &consumer_pid,
            &consumer_pid_,
            &provider_pid,
        ).await?;

        // persist model
        let callback = self
            .transfer_repo
            .put_transfer_callback(
                callback_id.unwrap(),
                EditTransferCallback {
                    ..Default::default()
                },
            )
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?;

        // data plane
        self.data_plane.on_transfer_completion(consumer_pid.clone()).await?;

        // return
        let mut transfer_process: TransferProcessMessage = callback.into();
        transfer_process.state = TransferState::COMPLETED;
        Ok(transfer_process)
    }

    async fn transfer_termination(
        &self,
        callback_id: Option<Urn>,
        consumer_pid: Urn,
        input: TransferTerminationMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        let TransferTerminationMessage { provider_pid, consumer_pid: consumer_pid_, .. } = input;
        self.do_validations(
            &callback_id,
            &consumer_pid,
            &consumer_pid_,
            &provider_pid,
        ).await?;

        // persist model
        let callback = self
            .transfer_repo
            .put_transfer_callback(
                callback_id.unwrap(),
                EditTransferCallback {
                    ..Default::default()
                },
            )
            .await
            .map_err(DSProtocolTransferConsumerErrors::DbErr)?;

        // data plane
        self.data_plane.on_transfer_termination(consumer_pid.clone()).await?;

        // return
        let mut transfer_process: TransferProcessMessage = callback.into();
        transfer_process.state = TransferState::TERMINATED;
        Ok(transfer_process)
    }
}
