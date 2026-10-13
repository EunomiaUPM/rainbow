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

//! The `ContractNegotiationError` a peer gets when a message is rejected (DSP 8.3.2).

use common::dsp_common::context_field::ContextField;
use common::dsp_common::DspErrorBody;
use serde::{Deserialize, Serialize};

/// `ContractNegotiationError` body: the pids it refers to, a code and every reason found.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NegotiationErrorDto {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: String,
    pub consumer_pid: String,
    pub provider_pid: String,
    #[serde(flatten)]
    pub error: DspErrorBody,
}

impl NegotiationErrorDto {
    pub const TYPE: &'static str = "ContractNegotiationError";

    pub fn new(
        consumer_pid: String,
        provider_pid: String,
        code: String,
        reason: Vec<String>,
    ) -> Self {
        Self {
            context: ContextField::default(),
            _type: Self::TYPE.to_string(),
            consumer_pid,
            provider_pid,
            error: DspErrorBody { code, reason },
        }
    }
}
