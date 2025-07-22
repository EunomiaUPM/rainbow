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

use crate::consumer::core::ds_protocol::ds_protocol_err::DSProtocolTransferConsumerErrors;
use thiserror::Error;
use urn::Urn;

#[derive(Debug, Error)]
pub enum DSRPCTransferConsumerErrors {
    #[error("Provider not reachable")]
    ProviderNotReachable {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Provider internal error")]
    ProviderInternalError {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
        error: Option<serde_json::Value>,
    },
    #[error("Provider response is not protocol compliant")]
    ProviderResponseNotSerializable {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Dataspace protocol error")]
    DSProtocolTransferConsumerError(DSProtocolTransferConsumerErrors),
}