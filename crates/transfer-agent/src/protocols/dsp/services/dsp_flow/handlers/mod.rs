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

//! What applying each inbound DSP message writes: one handler per message.

pub mod completion;
pub mod request;
pub mod start;
pub mod suspension;
pub mod termination;

use common::dsp_common::persistence::{DspPersistenceTrait, StateChange};
use ymir::errors::{Errors, Outcome};

use crate::entities::transfer_message::Direction;
use crate::entities::transfer_process::TransferProcess;
use crate::protocols::dsp::entities::context_common::TransferContextProcessSlot;
use crate::protocols::dsp::entities::context_dsp::TransferDspDomain;
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::entities::state_change::TransferMessageRecord;
use crate::protocols::dsp::services::dsp_persistence::service::TransferDspPersistence;

/// The state change a peer's message causes on an existing process.
pub struct InboundTransition;

impl InboundTransition {
    /// Moves the process to `target`, keeps the message, and leaves the new process in `domain`.
    pub async fn apply(
        persistence: &TransferDspPersistence,
        domain: &mut TransferDspDomain,
        target: TransferDSPState,
    ) -> Outcome<TransferProcess> {
        let TransferContextProcessSlot::Existing(current) = &domain.process else {
            return Err(Errors::crazy(
                "a state change needs an existing process",
                None,
            ));
        };
        let fields = &domain.typed.fields;
        let change = StateChange {
            target,
            code: fields.code.clone(),
            reason: (!fields.reason.is_empty()).then(|| fields.reason.clone()),
            message: TransferMessageRecord {
                message: domain.typed.message.clone(),
                direction: Direction::Inbound,
                envelope: domain.typed.envelope(),
            },
        };
        let process = persistence.change_process_state(current, change).await?;
        domain.process = TransferContextProcessSlot::Existing(Box::new(process.clone()));
        Ok(process)
    }
}
