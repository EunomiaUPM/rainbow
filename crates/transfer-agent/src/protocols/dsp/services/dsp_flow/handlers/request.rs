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

//! `TransferRequestMessage` received by the provider (DSP 9.2.1): opens the process.

use std::collections::HashMap;

use serde_json::json;
use url::Url;
use ymir::errors::{Errors, Outcome};

use crate::entities::commands::NewTransferProcessCommand;
use crate::entities::ids::{ParticipantId, TransferProcessId};
use crate::entities::protocol::{
    ProtocolId, ProtocolState, StateMetadata, TransferRole, CONSUMER_PID_KEY, PROVIDER_PID_KEY,
};
use crate::entities::message_envelope::MessageEnvelope;
use crate::entities::transfer_message::Direction;
use crate::protocols::dsp::entities::context_common::{
    TransferContextConnectorRole, TransferContextProcessSlot, DIRECTION_PROPERTY,
};
use crate::protocols::dsp::entities::context_dsp::{TransferDspDomain, TransferDspOutcome};
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::entities::state_metadata::TransferDSPStateAttribute;
use crate::protocols::dsp::services::transfer_writer::service::TransferWriter;

/// Creates the process with a new `providerPid`, records the message and starts the dataplane.
pub struct RequestHandler;

impl RequestHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        writer: &TransferWriter,
        mut domain: TransferDspDomain,
    ) -> Outcome<TransferDspOutcome> {
        let TransferContextProcessSlot::New { consumer_pid } = &domain.process else {
            return Err(Errors::crazy("a request opens a new process", None));
        };
        let consumer_pid = consumer_pid.clone();
        let provider_pid = format!("urn:uuid:{}", uuid::Uuid::new_v4());
        let requested = ProtocolState(TransferDSPState::REQUESTED.to_string().into());

        let cmd = Self::command(&domain, &consumer_pid, &provider_pid, &requested)?;
        let created = writer
            .processes
            .create(&TransferWriter::actor(), &cmd)
            .await?;
        let process = writer.reload(created.id.as_urn()).await?;
        let envelope: MessageEnvelope = domain.typed.envelope();
        writer
            .record(
                &process,
                Direction::Inbound,
                &domain.typed.message,
                envelope,
                (&requested, &requested),
            )
            .await?;

        // The provider side of the dataplane needs the stored process, so it runs after the write.
        domain.process = TransferContextProcessSlot::Existing(Box::new(process.clone()));
        writer
            .dataplane
            .on_transfer_request_post(&domain.dataplane_hooks()?)
            .await?;
        Ok(TransferDspOutcome::applied(
            &domain,
            &process,
            TransferDSPState::REQUESTED,
            true,
        ))
    }

    fn command(
        domain: &TransferDspDomain,
        consumer_pid: &str,
        provider_pid: &str,
        requested: &ProtocolState,
    ) -> Outcome<NewTransferProcessCommand> {
        let fields = &domain.typed.fields;
        let identifiers = HashMap::from([
            (CONSUMER_PID_KEY.to_string(), consumer_pid.to_string()),
            (PROVIDER_PID_KEY.to_string(), provider_pid.to_string()),
        ]);
        let connector_instance_id = match &domain.connector_instance {
            TransferContextConnectorRole::ProviderHavingConnector(c) => Some(c.id.to_string()),
            TransferContextConnectorRole::ConsumerNotHavingConnector => None,
        };
        Ok(NewTransferProcessCommand {
            id: Some(TransferProcessId::generate()),
            visibility: None,
            owner: Some(domain.owner()),
            role: TransferRole::Provider,
            protocol: ProtocolId::Dsp2025_1,
            initial_state: requested.clone(),
            initial_state_metadata: StateMetadata {
                attribute: Some(TransferDSPStateAttribute::OnRequest.to_string()),
                reason: None,
                code: None,
            },
            callback_address: fields
                .callback_address
                .as_deref()
                .and_then(|u| Url::parse(u).ok()),
            connector_instance_id,
            agreement_id: domain.agreement.id.clone(),
            peer_participant_id: ParticipantId::new(
                domain.typed.peer().participant_id.clone(),
            ),
            identifiers: Some(identifiers),
            properties: Some(json!({ DIRECTION_PROPERTY: domain.transfer_direction })),
        })
    }
}
