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

//! Code and reasons of a DSP error message, before each agent wraps them in its own error type
//! (`ContractNegotiationError`, `CatalogError`, `TransferError`).

use crate::validation::Violations;
use serde::{Deserialize, Serialize};

/// The `code` and `reason` members shared by every DSP error message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DspErrorBody {
    pub code: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<String>,
}

impl DspErrorBody {
    /// One `<path>: <message>` reason per violation, in the order they were found.
    pub fn from_violations(code: impl Into<String>, violations: &Violations) -> Self {
        Self {
            code: code.into(),
            reason: violations.to_reasons(),
        }
    }
}
