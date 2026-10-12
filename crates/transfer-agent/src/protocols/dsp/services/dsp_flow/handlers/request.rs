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
use ymir::errors::Outcome;

use crate::entities::commands::NewTransferProcessCommand;
use crate::entities::ids::{ParticipantId, TransferProcessId};
use crate::entities::protocol::{
    ProtocolId, ProtocolState, StateMetadata, TransferRole, CONSUMER_PID_KEY, PROVIDER_PID_KEY,
};
use crate::protocols::dsp::entities::ack::TransferProcessAck;
use crate::protocols::dsp::entities::context_common::{
    TransferContextConnectorRole, TransferContextProcessSlot,
};
use crate::protocols::dsp::entities::context_dsp::TransferDSPContextDomain;
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::entities::state_metadata::TransferDSPStateAttribute;
use crate::protocols::dsp::services::dsp_flow::handlers::TransferHandlerDeps;
use crate::protocols::dsp::services::dsp_flow::views::TransferDspReply;
use crate::protocols::dsp::services::dsp_domain_loader::dsp_domain_loader::DIRECTION_PROPERTY;

/// Creates the process with a new `providerPid`, records the message and starts the dataplane.
pub struct RequestHandler;

impl RequestHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        deps: &TransferHandlerDeps,
        mut facts: TransferDSPContextDomain,
    ) -> Outcome<TransferDspReply> {
        let consumer_pid = facts.process_urn("transfer request")?.to_string();
        let provider_pid = format!("urn:uuid:{}", uuid::Uuid::new_v4());
        let requested = ProtocolState(TransferDSPState::REQUESTED.to_string().into());

        let cmd = Self::command(&facts, &consumer_pid, &provider_pid, &requested)?;
        let created = deps
            .processes
            .create(&TransferHandlerDeps::actor(), &cmd)
            .await?;
        let process = deps.reload(created.id.as_urn()).await?;
        deps.record_message(&process, &facts, &requested, &requested)
            .await?;

        // The provider side of the dataplane needs the stored process, so it runs after the write.
        facts.process = TransferContextProcessSlot::Existing(process.clone());
        deps.dataplane
            .on_transfer_request_post(&facts.dataplane_hooks()?)
            .await?;

        Ok(TransferDspReply {
            ack: TransferProcessAck::new(consumer_pid, provider_pid, TransferDSPState::REQUESTED),
            created: true,
            process_version: process.version(),
            idempotency_key: facts.typed.idempotency_key.clone(),
            agreement_id: facts.agreement.id.to_string(),
            role: TransferRole::Provider,
        })
    }

    fn command(
        facts: &TransferDSPContextDomain,
        consumer_pid: &str,
        provider_pid: &str,
        requested: &ProtocolState,
    ) -> Outcome<NewTransferProcessCommand> {
        let fields = &facts.typed.fields;
        let identifiers = HashMap::from([
            (CONSUMER_PID_KEY.to_string(), consumer_pid.to_string()),
            (PROVIDER_PID_KEY.to_string(), provider_pid.to_string()),
        ]);
        let connector_instance_id = match &facts.connector_instance {
            TransferContextConnectorRole::ProviderHavingConnector(c) => Some(c.id.to_string()),
            TransferContextConnectorRole::ConsumerNotHavingConnector => None,
        };
        Ok(NewTransferProcessCommand {
            id: Some(TransferProcessId::generate()),
            visibility: None,
            owner: Some(facts.owner()),
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
            agreement_id: facts.agreement.id.clone(),
            peer_participant_id: ParticipantId::new(
                facts
                    .typed
                    .rdf
                    .parsed
                    .raw
                    .authn
                    .associated_participant
                    .participant_id
                    .clone(),
            ),
            identifiers: Some(identifiers),
            properties: Some(json!({ DIRECTION_PROPERTY: facts.transfer_direction })),
        })
    }
}
