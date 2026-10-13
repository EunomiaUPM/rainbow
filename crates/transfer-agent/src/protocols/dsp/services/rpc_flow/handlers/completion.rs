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

//! `TransferCompletionMessage` sent (DSP 9.2.4): the peer accepted our completion.

use serde_json::Value;
use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_rpc::{TransferRpcDomain, TransferRpcOutcome};
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::services::dsp_persistence::service::TransferDspPersistence;
use crate::protocols::dsp::services::rpc_flow::handlers::OutboundTransition;
use crate::setup::context::AppContext;

/// Moves our process to COMPLETED between the dataplane's completion hooks.
pub struct CompletionHandler;

impl CompletionHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        ctx: &AppContext,
        persistence: &TransferDspPersistence,
        domain: TransferRpcDomain,
        reply: &Value,
    ) -> Outcome<TransferRpcOutcome> {
        let hooks = OutboundTransition::hooks(&domain);
        ctx.ports.dataplane.on_transfer_completion_pre(&hooks).await?;
        let process =
            OutboundTransition::apply(persistence, &domain, TransferDSPState::COMPLETED).await?;
        ctx.ports.dataplane.on_transfer_completion_post(&hooks).await?;
        Ok(TransferRpcOutcome::sent(domain, reply, process))
    }
}
