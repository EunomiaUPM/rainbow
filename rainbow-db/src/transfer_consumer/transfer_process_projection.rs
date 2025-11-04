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

use crate::transfer_consumer::entities::transfer_callback;
use crate::transfer_consumer::entities::transfer_callback::Model;
use rainbow_common::protocol::context_field::ContextField;
use rainbow_common::protocol::transfer::transfer_consumer_process::TransferConsumerProcess;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::{TransferMessageTypes, TransferState};
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult, Clone)]
pub struct TransferConsumerProcessFromSQL {
    pub id: String,
    pub consumer_pid: String,
    pub provider_pid: Option<String>,
    pub associated_provider: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub data_plane_id: Option<String>,
    pub data_address: Option<serde_json::Value>,
    pub restart_flag: bool,
    pub message_type: Option<String>,
    pub from: String,
}

impl From<TransferConsumerProcessFromSQL> for TransferConsumerProcess {
    fn from(value: TransferConsumerProcessFromSQL) -> Self {
        let state = match value.message_type {
            None => "".to_string(),
            Some(s) => match s.parse::<TransferMessageTypes>().unwrap() {
                TransferMessageTypes::TransferRequestMessage => TransferState::REQUESTED.to_string(),
                TransferMessageTypes::TransferStartMessage => TransferState::STARTED.to_string(),
                TransferMessageTypes::TransferSuspensionMessage => TransferState::SUSPENDED.to_string(),
                TransferMessageTypes::TransferCompletionMessage => TransferState::COMPLETED.to_string(),
                TransferMessageTypes::TransferTerminationMessage => TransferState::TERMINATED.to_string(),
                _ => "".to_string(),
            },
        };
        let state_attribute = match value.from.as_str() {
            "Consumer" => "BY_CONSUMER".to_string(),
            _ => "BY_PROVIDER".to_string(),
        };
        Self {
            id: value.id,
            consumer_pid: value.consumer_pid,
            provider_pid: value.provider_pid,
            associated_provider: value.associated_provider,
            created_at: value.created_at,
            updated_at: value.updated_at,
            data_plane_id: value.data_plane_id,
            data_address: value.data_address,
            restart_flag: value.restart_flag,
            state,
            state_attribute,
        }
    }
}

impl From<TransferConsumerProcessFromSQL> for TransferProcessMessage {
    fn from(value: TransferConsumerProcessFromSQL) -> Self {
        let state = match value.message_type {
            None => TransferState::REQUESTED,
            Some(s) => match s.parse::<TransferMessageTypes>().unwrap() {
                TransferMessageTypes::TransferRequestMessage => TransferState::REQUESTED,
                TransferMessageTypes::TransferStartMessage => TransferState::STARTED,
                TransferMessageTypes::TransferSuspensionMessage => TransferState::SUSPENDED,
                TransferMessageTypes::TransferCompletionMessage => TransferState::COMPLETED,
                TransferMessageTypes::TransferTerminationMessage => TransferState::TERMINATED,
                _ => TransferState::TERMINATED,
            },
        };
        Self {
            context: ContextField::default(),
            _type: TransferMessageTypes::TransferProcessMessage.to_string(),
            provider_pid: value.provider_pid.unwrap_or("".to_string()),
            consumer_pid: value.consumer_pid,
            state,
        }
    }
}

impl Into<transfer_callback::Model> for TransferConsumerProcessFromSQL {
    fn into(self) -> Model {
        transfer_callback::Model {
            id: self.id,
            consumer_pid: self.consumer_pid,
            provider_pid: self.provider_pid,
            associated_provider: self.associated_provider,
            created_at: self.created_at,
            updated_at: self.updated_at,
            data_plane_id: self.data_plane_id,
            data_address: self.data_address,
            restart_flag: self.restart_flag,
        }
    }
}
