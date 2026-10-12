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

//! Start, completion, suspension and termination (DSP 9.2.2 to 9.2.5): move the process on.

use ymir::errors::{Errors, Outcome};

use crate::entities::commands::EditTransferProcessCommand;
use crate::entities::protocol::{ProtocolState, StateMetadata, TransferRole};
use crate::protocols::dsp::entities::ack::TransferProcessAck;
use crate::protocols::dsp::entities::context_common::TransferContextProcessSlot;
use crate::protocols::dsp::entities::context_dsp::TransferDSPContextDomain;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::entities::state_metadata::TransferDSPStateAttribute;
use crate::protocols::dsp::flow::handlers::TransferHandlerDeps;
use crate::protocols::dsp::flow::TransferDspReply;

/// Moves an existing process to the state its message names, around the dataplane hooks.
pub struct TransitionHandler;

impl TransitionHandler {
    #[tracing::instrument(level = "info", skip_all, err, fields(message = %facts.typed.message))]
    pub async fn handle(
        deps: &TransferHandlerDeps,
        mut facts: TransferDSPContextDomain,
    ) -> Outcome<TransferDspReply> {
        let TransferContextProcessSlot::Existing(current) = &facts.process else {
            return Err(Errors::crazy(
                "a state change needs an existing process",
                None,
            ));
        };
        let target = Self::target_state(&facts.typed.message)?;
        let from = current.state().clone();
        let to = ProtocolState(target.to_string().into());
        let id = current.id().as_urn().clone();

        Self::pre_hook(deps, &facts).await?;
        let edit = EditTransferProcessCommand {
            state: Some(to.clone()),
            state_metadata: Some(StateMetadata {
                attribute: Some(Self::caused_by(facts.role).to_string()),
                reason: (!facts.typed.fields.reason.is_empty())
                    .then(|| facts.typed.fields.reason.clone()),
                code: facts.typed.fields.code.clone(),
            }),
            identifiers: None,
            properties: None,
            error_details: None,
        };
        deps.processes
            .edit(&TransferHandlerDeps::actor(), &id, &edit)
            .await?;
        let process = deps.reload(&id).await?;
        deps.record_message(&process, &facts, &from, &to).await?;

        facts.process = TransferContextProcessSlot::Existing(process.clone());
        Self::post_hook(deps, &facts).await?;

        let correlation = process.correlation();
        Ok(TransferDspReply {
            ack: TransferProcessAck::new(
                correlation.consumer_pid.clone().unwrap_or_default(),
                correlation.provider_pid.clone().unwrap_or_default(),
                target,
            ),
            created: false,
            process_version: process.version(),
            idempotency_key: facts.typed.idempotency_key.clone(),
            agreement_id: facts.agreement.id.to_string(),
            role: facts.role,
        })
    }

    fn target_state(message: &TransferDSPMessageType) -> Outcome<TransferDSPState> {
        use TransferDSPMessageType::*;
        match message {
            TransferStartMessage => Ok(TransferDSPState::STARTED),
            TransferCompletionMessage => Ok(TransferDSPState::COMPLETED),
            TransferSuspensionMessage => Ok(TransferDSPState::SUSPENDED),
            TransferTerminationMessage => Ok(TransferDSPState::TERMINATED),
            other => Err(Errors::crazy(
                format!("{other} is not a state change"),
                None,
            )),
        }
    }

    /// The peer sent the message, so the change is caused by the role opposite to ours.
    fn caused_by(our_role: TransferRole) -> TransferDSPStateAttribute {
        match our_role {
            TransferRole::Provider => TransferDSPStateAttribute::ByConsumer,
            _ => TransferDSPStateAttribute::ByProvider,
        }
    }

    async fn pre_hook(deps: &TransferHandlerDeps, facts: &TransferDSPContextDomain) -> Outcome<()> {
        use TransferDSPMessageType::*;
        let hooks = facts.dataplane_hooks()?;
        match facts.typed.message {
            TransferStartMessage => deps.dataplane.on_transfer_start_pre(&hooks).await?,
            TransferCompletionMessage => deps.dataplane.on_transfer_completion_pre(&hooks).await?,
            TransferSuspensionMessage => deps.dataplane.on_transfer_suspension_pre(&hooks).await?,
            TransferTerminationMessage => {
                deps.dataplane.on_transfer_termination_pre(&hooks).await?
            }
            _ => None,
        };
        Ok(())
    }

    async fn post_hook(
        deps: &TransferHandlerDeps,
        facts: &TransferDSPContextDomain,
    ) -> Outcome<()> {
        use TransferDSPMessageType::*;
        let hooks = facts.dataplane_hooks()?;
        match facts.typed.message {
            TransferStartMessage => deps.dataplane.on_transfer_start_post(&hooks).await?,
            TransferCompletionMessage => deps.dataplane.on_transfer_completion_post(&hooks).await?,
            TransferSuspensionMessage => deps.dataplane.on_transfer_suspension_post(&hooks).await?,
            TransferTerminationMessage => {
                deps.dataplane.on_transfer_termination_post(&hooks).await?
            }
            _ => None,
        };
        Ok(())
    }
}
