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

use crate::provider::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use serde_json::Value;
use thiserror::Error;
use urn::Urn;

#[derive(Debug, Error)]
pub enum DSRPCContractNegotiationProviderErrors {
    #[error("Consumer not reachable")]
    ConsumerNotReachable {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Consumer internal error")]
    ConsumerInternalError {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
        consumer_error: Value,
    },
    #[error("Consumer response is not protocol compliant")]
    ConsumerResponseNotSerializable {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Dataspace protocol error")]
    DSProtocolContractNegotiationError(IdsaCNError),
    #[error("Consumer and Provider not coincide")]
    ConsumerAndProviderCorrelationError {
        provider_pid: Urn,
        consumer_pid: Urn,
    },
    #[error("Consumer and Provider not coincide")]
    OdrlValidationError,
}