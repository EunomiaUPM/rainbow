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

use super::entities::transfer_process;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;

impl From<transfer_process::Model> for TransferProcessMessage {
    fn from(model: transfer_process::Model) -> Self {
        TransferProcessMessage {
            provider_pid: model.provider_pid,
            consumer_pid: model.consumer_pid.unwrap(),
            state: model.state.parse().unwrap(),
            ..Default::default()
        }
    }
}
