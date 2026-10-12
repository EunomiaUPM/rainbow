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

//! What applying each inbound DSP message writes: one handler opens a process, the other
//! moves it to its next state.

pub mod request;
pub mod transition;

use std::sync::Arc;

use common::oauth::{OwnerScope, UserInfo};
use urn::Urn;
use ymir::errors::{Errors, Outcome};

use crate::data::repo::transfer_process::TransferProcessRepoTrait;
use crate::entities::commands::NewTransferMessageCommand;
use crate::entities::message_envelope::MessageEnvelope;
use crate::entities::protocol::{ProtocolId, ProtocolMessageType, ProtocolState};
use crate::entities::transfer_message::Direction;
use crate::entities::transfer_process::TransferProcess;
use crate::protocols::dsp::entities::context_dsp::TransferDspFacts;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::facades::dataplane_facade::DataPlaneFacadeTrait;
use crate::services::transfer_message::TransferMessageServiceTrait;
use crate::services::transfer_process::TransferProcessServiceTrait;

/// Services the handlers write through.
#[derive(Clone)]
pub struct TransferHandlerDeps {
    pub processes: Arc<dyn TransferProcessServiceTrait>,
    pub process_repo: Arc<dyn TransferProcessRepoTrait>,
    pub messages: Arc<dyn TransferMessageServiceTrait>,
    pub dataplane: Arc<dyn DataPlaneFacadeTrait>,
}

impl TransferHandlerDeps {
    /// Who the protocol writes as: an in-process flow on a process it already authorized.
    pub(crate) fn actor() -> UserInfo {
        UserInfo::system()
    }

    /// Reads the process back as stored, for the dataplane hooks and the reply.
    pub(crate) async fn reload(&self, id: &Urn) -> Outcome<TransferProcess> {
        self.process_repo
            .get_transfer_process_by_id(&OwnerScope::All, id)
            .await?
            .ok_or_else(|| {
                Errors::crazy(
                    format!("transfer process {id} vanished after a write"),
                    None,
                )
            })
    }

    /// Keeps the inbound message, with its canonical form, in the process history.
    async fn record_message(
        &self,
        process: &TransferProcess,
        facts: &TransferDspFacts,
        from: &ProtocolState,
        to: &ProtocolState,
    ) -> Outcome<()> {
        self.record(
            process,
            Direction::Inbound,
            &facts.input.body.message,
            MessageEnvelope::from_rdf(&facts.input.body),
            (from, to),
        )
        .await
    }

    /// Adds a message, either way, to the process history.
    pub(crate) async fn record(
        &self,
        process: &TransferProcess,
        direction: Direction,
        message: &TransferDSPMessageType,
        envelope: MessageEnvelope,
        (from, to): (&ProtocolState, &ProtocolState),
    ) -> Outcome<()> {
        let cmd = NewTransferMessageCommand {
            id: None,
            transfer_process_id: process.id().clone(),
            visibility: None,
            owner: Some(process.owner().clone()),
            direction,
            protocol: ProtocolId::Dsp2025_1,
            message_type: ProtocolMessageType(message.to_string().into()),
            state_transition_from: from.clone(),
            state_transition_to: to.clone(),
            envelope,
        };
        self.messages.create(&Self::actor(), &cmd).await?;
        Ok(())
    }
}
