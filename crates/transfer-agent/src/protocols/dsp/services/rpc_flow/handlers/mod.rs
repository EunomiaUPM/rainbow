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

//! What sending each outbound message writes once the peer accepted it: one handler per message.

pub mod completion;
pub mod request;
pub mod start;
pub mod suspension;
pub mod termination;

use ymir::errors::{Errors, Outcome};

use crate::entities::message_envelope::MessageEnvelope;
use crate::entities::protocol::TransferRole;
use crate::entities::transfer_message::Direction;
use crate::entities::transfer_process::TransferProcess;
use crate::protocols::dsp::entities::context_rpc::TransferRpcDomain;
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::entities::state_change::TransferStateChange;
use crate::protocols::dsp::entities::state_metadata::TransferDSPStateAttribute;
use crate::protocols::dsp::facades::dataplane_facade::DataplaneHookContext;
use crate::protocols::dsp::services::transfer_writer::service::TransferWriter;

/// The state change our own message causes on an existing process.
pub struct OutboundTransition;

impl OutboundTransition {
    /// Moves the process to `target` and keeps the message as sent.
    pub async fn apply(
        writer: &TransferWriter,
        domain: &TransferRpcDomain,
        target: TransferDSPState,
    ) -> Outcome<TransferProcess> {
        let current = domain
            .process
            .as_ref()
            .ok_or_else(|| Errors::crazy("a state change needs a process", None))?;
        let change = TransferStateChange {
            target,
            attribute: Self::caused_by(domain.role),
            reason: domain.typed.body.reason.clone(),
            code: domain.typed.body.code.clone(),
            direction: Direction::Outbound,
            message: domain.typed.message.clone(),
            envelope: MessageEnvelope::from_rdf(&domain.outgoing),
        };
        writer.change_state(current, change).await
    }

    /// The hooks around a change, with the data address the message hands the peer.
    pub fn hooks(domain: &TransferRpcDomain) -> DataplaneHookContext {
        domain.hooks(domain.outgoing.fields.data_address.clone())
    }

    /// We sent the message, so the change is caused by our own role.
    fn caused_by(our_role: TransferRole) -> TransferDSPStateAttribute {
        match our_role {
            TransferRole::Provider => TransferDSPStateAttribute::ByProvider,
            _ => TransferDSPStateAttribute::ByConsumer,
        }
    }
}
