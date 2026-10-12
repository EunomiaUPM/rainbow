/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! What a local user asks the agent to send, as the RPC body carries it.

use serde::{Deserialize, Serialize};

use crate::protocols::dsp::entities::data_address::DataAddressDto;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;

/// RPC body for any outbound transfer message; which fields matter depends on the message.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRpcBody {
    /// The provider's participant id, for a request.
    pub associated_agent_peer: Option<String>,
    pub agreement_id: Option<String>,
    pub format: Option<String>,
    /// The sink a push transfer delivers to, for a request.
    pub data_address: Option<DataAddressDto>,
    /// The provider's DSP base, for a request.
    pub provider_address: Option<String>,
    /// Our DSP base as the provider will call it back; defaults to this agent's own.
    pub callback_address: Option<String>,
    pub consumer_pid: Option<String>,
    pub provider_pid: Option<String>,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

/// The message to send, fixed by the route, and its body.
#[derive(Debug, Clone)]
pub struct TransferRpcCommand {
    pub message: TransferDSPMessageType,
    pub body: TransferRpcBody,
}

impl TransferRpcCommand {
    /// The pid of an existing process the command refers to, ours or the peer's.
    pub fn pid(&self) -> Option<&str> {
        self.body
            .provider_pid
            .as_deref()
            .or(self.body.consumer_pid.as_deref())
    }
}
