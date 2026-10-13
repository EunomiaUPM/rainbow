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

//! The write side shared by the inbound and outbound flows.

use std::sync::Arc;

use common::oauth::{OwnerScope, UserInfo};
use urn::Urn;
use ymir::errors::{Errors, Outcome};

use crate::data::repo::transfer_process::TransferProcessRepoTrait;
use crate::entities::commands::{EditTransferProcessCommand, NewTransferMessageCommand};
use crate::entities::message_envelope::MessageEnvelope;
use crate::entities::protocol::{ProtocolId, ProtocolMessageType, ProtocolState, StateMetadata};
use crate::entities::transfer_message::Direction;
use crate::entities::transfer_process::TransferProcess;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::state_change::TransferStateChange;
use crate::protocols::dsp::facades::dataplane_facade::DataPlaneFacadeTrait;
use crate::services::transfer_message::TransferMessageServiceTrait;
use crate::services::transfer_process::TransferProcessServiceTrait;

/// Writes processes and their history, and drives the dataplane hooks around them.
#[derive(Clone)]
pub struct TransferWriter {
    pub processes: Arc<dyn TransferProcessServiceTrait>,
    pub process_repo: Arc<dyn TransferProcessRepoTrait>,
    pub messages: Arc<dyn TransferMessageServiceTrait>,
    pub dataplane: Arc<dyn DataPlaneFacadeTrait>,
}

impl TransferWriter {
    /// Who the protocol writes as: an in-process flow on a process it already authorized.
    pub fn actor() -> UserInfo {
        UserInfo::system()
    }

    /// Reads the process back as stored, for the dataplane hooks and the outcome.
    pub async fn reload(&self, id: &Urn) -> Outcome<TransferProcess> {
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

    /// Moves `current` to the change's state and keeps the message that caused it.
    pub async fn change_state(
        &self,
        current: &TransferProcess,
        change: TransferStateChange,
    ) -> Outcome<TransferProcess> {
        let id = current.id().as_urn().clone();
        let (from, to) = (
            current.state().clone(),
            ProtocolState(change.target.to_string().into()),
        );
        let edit = EditTransferProcessCommand {
            state: Some(to.clone()),
            state_metadata: Some(StateMetadata {
                attribute: Some(change.attribute.to_string()),
                reason: change.reason,
                code: change.code,
            }),
            identifiers: None,
            properties: None,
            error_details: None,
        };
        self.processes.edit(&Self::actor(), &id, &edit).await?;
        let process = self.reload(&id).await?;
        self.record(
            &process,
            change.direction,
            &change.message,
            change.envelope,
            (&from, &to),
        )
        .await?;
        Ok(process)
    }

    /// Adds a message, either way, to the process history.
    pub async fn record(
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
