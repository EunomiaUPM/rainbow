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
use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;
use rainbow_db::transfer_consumer::repo::{EditTransferCallback, NewTransferCallback};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NewTransferConsumerRequest {
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

impl Default for NewTransferConsumerRequest {
    fn default() -> Self {
        NewTransferConsumerRequest { data_address: None }
    }
}

impl Into<NewTransferCallback> for NewTransferConsumerRequest {
    fn into(self) -> NewTransferCallback {
        NewTransferCallback {
            callback_id: None,
            consumer_pid: None,
            provider_pid: None,
            data_address: self.data_address.map(|data_address| serde_json::to_value(data_address).unwrap()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EditTransferConsumerRequest {
    #[serde(rename = "consumerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_pid: Option<Urn>,
    #[serde(rename = "providerPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_pid: Option<Urn>,
    #[serde(rename = "dataPlaneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_plane_id: Option<Urn>,
    #[serde(rename = "dataAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_address: Option<DataAddress>,
}

impl Into<EditTransferCallback> for EditTransferConsumerRequest {
    fn into(self) -> EditTransferCallback {
        EditTransferCallback {
            consumer_pid: self.consumer_pid,
            provider_pid: self.provider_pid,
            data_plane_id: self.data_plane_id,
            data_address: self.data_address.map(|data_address| serde_json::to_value(data_address).unwrap()),
        }
    }
}

impl Default for EditTransferConsumerRequest {
    fn default() -> Self {
        EditTransferConsumerRequest { consumer_pid: None, provider_pid: None, data_plane_id: None, data_address: None }
    }
}
