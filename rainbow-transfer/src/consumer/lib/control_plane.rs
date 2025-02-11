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

use axum::Json;
use rainbow_common::protocol::transfer::{
    TransferCompletionMessage, TransferStartMessage, TransferSuspensionMessage,
    TransferTerminationMessage,
};
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::transfer_consumer::repo::{EditTransferCallback, TRANSFER_CONSUMER_REPO};
use serde_json::to_value;
use urn::Urn;

pub async fn transfer_start(
    Json(input): Json<&TransferStartMessage>,
    callback: Urn,
    consumer_pid: Urn,
) -> anyhow::Result<()> {
    let callback = TRANSFER_CONSUMER_REPO
        .put_transfer_callback(
            callback,
            EditTransferCallback {
                provider_pid: Option::from(get_urn_from_string(&input.provider_pid)?),
                data_address: Option::from(to_value(input.data_address.clone())?),
                ..Default::default()
            },
        )
        .await?;
    
    Ok(())
}

pub fn transfer_completion(
    Json(input): Json<&TransferCompletionMessage>,
    callback: Urn,
    consumer_pid: Urn,
) -> anyhow::Result<()> {
    // will retrieve an error from middleware
    Ok(())
}

pub fn transfer_termination(
    Json(input): Json<&TransferTerminationMessage>,
    callback: Urn,
    consumer_pid: Urn,
) -> anyhow::Result<()> {
    // will retrieve an error from middleware
    Ok(())
}

pub fn transfer_suspension(
    Json(input): Json<&TransferSuspensionMessage>,
    callback: Urn,
    consumer_pid: Urn,
) -> anyhow::Result<()> {
    // will retrieve an error from middleware
    Ok(())
}
