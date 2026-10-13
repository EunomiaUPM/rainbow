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

//! What the outbound transfer RPC answers, as a eunomia `RpcTransferResult`.

use std::collections::HashMap;

use common::eunomia::EUNOMIA_CONTEXT_URL;
use serde::Serialize;
use serde_json::Value;

use crate::protocols::dsp::entities::context_rpc::TransferRpcOutcome;
use crate::services::transfer_process::views::TransferProcessView;

/// What was asked, what the peer said and the process as it stands.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRpcResult {
    #[serde(rename = "@context")]
    pub context: &'static str,
    #[serde(rename = "@type")]
    pub kind: &'static str,
    pub request: Value,
    pub response: Value,
    pub transfer_agent_model: TransferProcessView,
}

impl TransferRpcResult {
    pub const TYPE: &'static str = "RpcTransferResult";

    pub fn assemble(outcome: TransferRpcOutcome) -> Self {
        Self {
            context: EUNOMIA_CONTEXT_URL,
            kind: Self::TYPE,
            request: outcome.request,
            response: outcome.response,
            transfer_agent_model: TransferProcessView::assemble(outcome.process, HashMap::new()),
        }
    }
}
