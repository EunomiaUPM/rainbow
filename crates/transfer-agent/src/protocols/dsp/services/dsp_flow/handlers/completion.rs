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

//! `TransferCompletionMessage` received (DSP 9.2.4): the transfer is complete.

use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_dsp::{TransferDspDomain, TransferDspOutcome};
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::services::dsp_flow::handlers::InboundTransition;
use crate::protocols::dsp::services::dsp_persistence::service::TransferDspPersistence;
use crate::setup::context::AppContext;

/// Moves the process to COMPLETED between the dataplane's completion hooks.
pub struct CompletionHandler;

impl CompletionHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        ctx: &AppContext,
        persistence: &TransferDspPersistence,
        mut domain: TransferDspDomain,
    ) -> Outcome<TransferDspOutcome> {
        ctx.ports
            .dataplane
            .on_transfer_completion_pre(&domain.dataplane_hooks()?)
            .await?;
        let process =
            InboundTransition::apply(persistence, &mut domain, TransferDSPState::COMPLETED).await?;
        ctx.ports
            .dataplane
            .on_transfer_completion_post(&domain.dataplane_hooks()?)
            .await?;
        Ok(TransferDspOutcome::applied(
            &domain,
            &process,
            TransferDSPState::COMPLETED,
            false,
        ))
    }
}
