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

use crate::contracts_consumer::entities::cn_process;
use crate::contracts_consumer::repo::CnErrors;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_error::ContractErrorMessage;

impl From<cn_process::Model> for ContractAckMessage {
    fn from(model: cn_process::Model) -> Self {
        ContractAckMessage {
            provider_pid: model.provider_id.unwrap(),
            consumer_pid: model.consumer_id,
            ..Default::default()
        }
    }
}

impl From<CnErrors> for ContractErrorMessage {
    fn from(error: CnErrors) -> Self {
        match error {
            e @ CnErrors::CNProcessNotFound => Self {
                code: Some("NOT_FOUND".to_string()),
                reason: Some(vec![e.to_string()]),
                ..Default::default()
            },
            e => Self {
                code: Some("DB_ERROR".to_string()),
                reason: Some(vec![e.to_string()]),
                ..Default::default()
            }
        }
    }
}