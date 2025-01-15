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

use crate::common::utils::{has_data_address_in_push, is_agreement_valid};
use crate::provider::lib::data_plane::resolve_endpoint_from_agreement;
use anyhow::{bail, Error};
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::FormatAction;
use rainbow_common::err::transfer_err::TransferErrorType;
use rainbow_common::protocol::transfer::{
    DataAddress, TransferCompletionMessage, TransferProcessMessage, TransferRequestMessage,
    TransferRoles, TransferStartMessage, TransferStateForDb, TransferSuspensionMessage,
    TransferTerminationMessage,
};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_dataplane::core::DataPlanePeerCreationBehavior;
use rainbow_dataplane::{
    bootstrap_data_plane_in_provider, connect_to_streaming_service,
    disconnect_from_streaming_service, set_data_plane_next_hop,
};
use rainbow_db::transfer_provider::repo::{
    EditTransferProcessModel, NewTransferMessageModel, NewTransferProcessModel,
    TRANSFER_PROVIDER_REPO,
};
use std::future::{Future, IntoFuture};
use std::str::FromStr;
use urn::Urn;

pub async fn get_transfer_requests_by_provider(
    provider_pid: Urn,
) -> anyhow::Result<Option<TransferProcessMessage>> {
    let transfers = TRANSFER_PROVIDER_REPO.get_transfer_process_by_provider(provider_pid).await?;
    let transfers = transfers.map(|t| TransferProcessMessage::from(t));
    Ok(transfers)
}

pub async fn transfer_request<F, Fut, M>(
    input: TransferRequestMessage,
    callback: F,
) -> anyhow::Result<TransferProcessMessage>
where
    F: Fn(M, Urn, Option<DataAddress>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Result<(), Error>> + Send,
    M: From<TransferRequestMessage> + Send + 'static,
{
    let db_connection = get_db_connection().await;

    // agreement validation - validate
    if is_agreement_valid(&input.agreement_id)? == false {
        bail!(TransferErrorType::AgreementError);
    }

    // dct:format is push, dataAdress must be
    if has_data_address_in_push(&input.data_address, &input.format)? == false {
        bail!(TransferErrorType::DataAddressCannotBeNullOnPushError);
    }

    let provider_pid = get_urn(None);
    let consumer_pid = get_urn_from_string(&input.consumer_pid)?;
    let created_at = chrono::Utc::now().naive_utc();
    let message_type = input._type.clone();

    // data plane provision
    let agreement_id = get_urn_from_string(&input.agreement_id)?;
    let data_service = resolve_endpoint_from_agreement(agreement_id.clone()).await?;

    let data_plane_peer = bootstrap_data_plane_in_provider(input.clone(), provider_pid.clone())
        .await?
        .add_attribute(
            "endpointUrl".to_string(),
            data_service.dcat.clone().endpoint_url,
        )
        .add_attribute(
            "endpointDescription".to_string(),
            data_service.dcat.clone().endpoint_description,
        );
    let data_plane_peer =
        set_data_plane_next_hop(data_plane_peer, provider_pid.clone(), consumer_pid.clone()).await?;
    let data_plane_id = data_plane_peer.id.clone();
    connect_to_streaming_service(data_plane_id.clone()).await?;


    // db persist
    let transfer_process_db = TRANSFER_PROVIDER_REPO
        .create_transfer_process(NewTransferProcessModel {
            provider_pid: provider_pid.clone(),
            consumer_pid,
            agreement_id,
            data_plane_id,
        })
        .await?;

    let _ = TRANSFER_PROVIDER_REPO
        .create_transfer_message(
            provider_pid.clone(),
            NewTransferMessageModel {
                message_type,
                from: TransferRoles::Consumer,
                to: TransferRoles::Provider,
                content: serde_json::to_value(&input)?,
            },
        )
        .await?;

    // prepare data address for transfer start message
    let data_address = match input.clone().format.action {
        FormatAction::Push => None,
        FormatAction::Pull => Some(DataAddress {
            _type: "dspace:DataAddress".to_string(),
            endpoint_type: "HTTP".to_string(),
            endpoint: data_service.dcat.endpoint_description.to_string(),
            endpoint_properties: vec![],
        }),
    };

    // callback for sending after a transfer start
    callback(input.into(), provider_pid, data_address).await?;

    // return
    let tp = TransferProcessMessage::from(transfer_process_db);
    Ok(tp)
}

pub async fn transfer_start(input: TransferStartMessage) -> anyhow::Result<TransferProcessMessage> {
    // persist process
    let provider_pid = get_urn_from_string(&input.provider_pid)?;
    let transfer_process_db = TRANSFER_PROVIDER_REPO
        .put_transfer_process(
            provider_pid.clone(),
            EditTransferProcessModel {
                state: Option::from(TransferStateForDb::STARTED),
                ..Default::default()
            },
        )
        .await?;

    // create message
    let _ = TRANSFER_PROVIDER_REPO
        .create_transfer_message(
            provider_pid,
            NewTransferMessageModel {
                message_type: input._type.clone(),
                from: TransferRoles::Consumer,
                to: TransferRoles::Provider,
                content: serde_json::to_value(&input)?,
            },
        )
        .await?;

    let tp = TransferProcessMessage::from(transfer_process_db.clone());
    // data plane
    let data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.unwrap())?;
    connect_to_streaming_service(data_plane_id).await?;

    Ok(tp)
}

pub async fn transfer_suspension(
    input: TransferSuspensionMessage,
) -> anyhow::Result<TransferProcessMessage> {
    // persist process
    let provider_pid = get_urn_from_string(&input.provider_pid)?;

    let transfer_process_db = TRANSFER_PROVIDER_REPO
        .put_transfer_process(
            provider_pid.clone(),
            EditTransferProcessModel {
                state: Option::from(TransferStateForDb::SUSPENDED),
                ..Default::default()
            },
        )
        .await?;
    // create message
    let _ = TRANSFER_PROVIDER_REPO
        .create_transfer_message(
            provider_pid,
            NewTransferMessageModel {
                message_type: input._type.clone(),
                from: TransferRoles::Consumer,
                to: TransferRoles::Provider,
                content: serde_json::to_value(&input)?,
            },
        )
        .await?;

    let tp = TransferProcessMessage::from(transfer_process_db.clone());
    // data plane
    let data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.unwrap())?;
    disconnect_from_streaming_service(data_plane_id).await?;
    Ok(tp)
}

pub async fn transfer_completion(
    input: TransferCompletionMessage,
) -> anyhow::Result<TransferProcessMessage> {
    // persist process
    let provider_pid = get_urn_from_string(&input.provider_pid)?;

    let transfer_process_db = TRANSFER_PROVIDER_REPO
        .put_transfer_process(
            provider_pid.clone(),
            EditTransferProcessModel {
                state: Option::from(TransferStateForDb::COMPLETED),
                ..Default::default()
            },
        )
        .await?;

    // create message
    let _ = TRANSFER_PROVIDER_REPO
        .create_transfer_message(
            provider_pid,
            NewTransferMessageModel {
                message_type: input._type.clone(),
                from: TransferRoles::Consumer,
                to: TransferRoles::Provider,
                content: serde_json::to_value(&input)?,
            },
        )
        .await?;

    let tp = TransferProcessMessage::from(transfer_process_db.clone());

    // data plane
    let data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.unwrap())?;
    disconnect_from_streaming_service(data_plane_id).await?;
    Ok(tp)
}

pub async fn transfer_termination(
    input: TransferTerminationMessage,
) -> anyhow::Result<TransferProcessMessage> {
    // persist process
    let provider_pid = get_urn_from_string(&input.provider_pid)?;

    let transfer_process_db = TRANSFER_PROVIDER_REPO
        .put_transfer_process(
            provider_pid.clone(),
            EditTransferProcessModel {
                state: Option::from(TransferStateForDb::TERMINATED),
                ..Default::default()
            },
        )
        .await?;

    // create message
    let _ = TRANSFER_PROVIDER_REPO
        .create_transfer_message(
            provider_pid,
            NewTransferMessageModel {
                message_type: input._type.clone(),
                from: TransferRoles::Consumer,
                to: TransferRoles::Provider,
                content: serde_json::to_value(&input)?,
            },
        )
        .await?;

    let tp = TransferProcessMessage::from(transfer_process_db.clone());

    // data plane
    let data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.unwrap())?;
    disconnect_from_streaming_service(data_plane_id).await?;
    Ok(tp)
}
