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

//! The `TransferError` a peer gets when a message is rejected (DSP 9.3.2).

use common::dsp_common::context_field::ContextField;
use serde::{Deserialize, Serialize};

use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;

/// `TransferError` body: the pids it refers to, a code and every reason found.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferErrorDto {
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: TransferDSPMessageType,
    pub consumer_pid: String,
    pub provider_pid: String,
    pub code: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<String>,
}

impl TransferErrorDto {
    pub fn new(
        consumer_pid: String,
        provider_pid: String,
        code: String,
        reason: Vec<String>,
    ) -> Self {
        Self {
            context: ContextField::default(),
            _type: TransferDSPMessageType::TransferError,
            consumer_pid,
            provider_pid,
            code,
            reason,
        }
    }
}
