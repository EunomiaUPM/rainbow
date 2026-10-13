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

//! The `ContractNegotiation` ACK a peer gets when a message was applied (DSP 8.3.1).

use common::dsp_common::context_field::ContextField;
use serde::{Deserialize, Serialize};

use crate::protocols::dsp::entities::state::NegotiationState;

/// `ContractNegotiation` acknowledgement, with both pids and the state; all are required.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NegotiationAck {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    pub consumer_pid: String,
    pub provider_pid: String,
    pub state: NegotiationState,
}

impl NegotiationAck {
    pub const TYPE: &'static str = "ContractNegotiation";

    pub fn new(consumer_pid: String, provider_pid: String, state: NegotiationState) -> Self {
        Self {
            context: ContextField::default(),
            _type: Self::TYPE.to_string(),
            consumer_pid,
            provider_pid,
            state,
        }
    }
}
