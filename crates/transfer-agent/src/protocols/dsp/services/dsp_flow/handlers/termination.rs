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

//! `TransferTerminationMessage` received (DSP 9.2.5): the transfer ends for good.

use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_dsp::{TransferDspDomain, TransferDspOutcome};
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::services::dsp_flow::handlers::InboundTransition;
use crate::protocols::dsp::services::transfer_writer::service::TransferWriter;

/// Moves the process to TERMINATED between the dataplane's termination hooks.
pub struct TerminationHandler;

impl TerminationHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        writer: &TransferWriter,
        mut domain: TransferDspDomain,
    ) -> Outcome<TransferDspOutcome> {
        writer
            .dataplane
            .on_transfer_termination_pre(&domain.dataplane_hooks()?)
            .await?;
        let process =
            InboundTransition::apply(writer, &mut domain, TransferDSPState::TERMINATED).await?;
        writer
            .dataplane
            .on_transfer_termination_post(&domain.dataplane_hooks()?)
            .await?;
        Ok(TransferDspOutcome::applied(
            &domain,
            &process,
            TransferDSPState::TERMINATED,
            false,
        ))
    }
}
