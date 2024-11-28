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

use crate::transfer_provider::entities::transfer_process;
use rainbow_common::protocol::transfer::{
    TransferMessageTypes, TransferProcessMessage, TransferState, TRANSFER_CONTEXT,
};
use rainbow_common::utils::convert_uuid_to_uri;

pub mod entities;
pub mod migrations;
pub mod repo;

impl From<transfer_process::Model> for TransferProcessMessage {
    fn from(model: transfer_process::Model) -> Self {
        TransferProcessMessage {
            context: TRANSFER_CONTEXT.to_string(),
            _type: TransferMessageTypes::TransferProcessMessage.to_string(),
            provider_pid: convert_uuid_to_uri(&model.provider_pid).unwrap(),
            consumer_pid: convert_uuid_to_uri(&model.consumer_pid.unwrap()).unwrap(),
            state: TransferState::from(model.state),
        }
    }
}
