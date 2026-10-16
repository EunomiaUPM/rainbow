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

//! `TransferRequestMessage` sent (DSP 9.2.1): the provider accepted it, so our consumer process
//! is stored with both pids.

use std::collections::HashMap;
use std::str::FromStr;
use common::dsp_common::persistence::{DspPersistenceTrait, RecordedMessage};

use common::oauth::UserInfo;
use serde_json::{json, Value};
use url::Url;
use urn::Urn;
use ymir::errors::{BadFormat, Errors, Outcome};

use crate::entities::commands::NewTransferProcessCommand;
use crate::entities::ids::{ParticipantId, TransferProcessId};
use crate::entities::message_envelope::MessageEnvelope;
use crate::entities::protocol::{
    ProtocolId, ProtocolState, StateMetadata, TransferRole, CONSUMER_PID_KEY, PROVIDER_PID_KEY,
};
use crate::entities::transfer_message::Direction;
use crate::protocols::dsp::entities::context_common::DIRECTION_PROPERTY;
use crate::protocols::dsp::entities::context_rpc::{TransferRpcDomain, TransferRpcOutcome};
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::entities::state_change::TransferMessageRecord;
use crate::protocols::dsp::entities::state_metadata::TransferDSPStateAttribute;
use crate::protocols::dsp::services::dsp_persistence::service::TransferDspPersistence;
use crate::setup::context::AppContext;

/// Creates the consumer process with the provider's pid, records the request and runs the
/// dataplane's request hook.
pub struct RequestHandler;

impl RequestHandler {
    #[tracing::instrument(level = "info", skip_all, err)]
    pub async fn handle(
        ctx: &AppContext,
        persistence: &TransferDspPersistence,
        caller: &UserInfo,
        domain: TransferRpcDomain,
        reply: &Value,
    ) -> Outcome<TransferRpcOutcome> {
        let requested = ProtocolState(TransferDSPState::REQUESTED.to_string().into());
        let cmd = Self::command(&domain, reply, &requested)?;
        let process = persistence.open_process(caller, &cmd).await?;
        let record = TransferMessageRecord {
            message: domain.typed.message.clone(),
            direction: Direction::Outbound,
            envelope: MessageEnvelope::from_rdf(&domain.outgoing),
        };
        persistence
            .record_message(
                &process,
                RecordedMessage {
                    message: record,
                    from: None,
                    to: TransferDSPState::REQUESTED,
                },
            )
            .await?;
        ctx.ports
            .dataplane
            .on_transfer_request_post(&domain.hooks(domain.outgoing.fields.data_address.clone()))
            .await?;
        Ok(TransferRpcOutcome::sent(domain, reply, process))
    }

    fn command(
        domain: &TransferRpcDomain,
        reply: &Value,
        requested: &ProtocolState,
    ) -> Outcome<NewTransferProcessCommand> {
        let body = &domain.typed.body;
        let consumer_pid = domain
            .outgoing
            .fields
            .consumer_pid
            .clone()
            .unwrap_or_default();
        let provider_pid = reply["fields"]["providerPid"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        Ok(NewTransferProcessCommand {
            id: Some(TransferProcessId::new(domain.process_id.clone())),
            visibility: None,
            owner: None,
            role: TransferRole::Consumer,
            protocol: ProtocolId::Dsp2025_1,
            initial_state: requested.clone(),
            initial_state_metadata: StateMetadata {
                attribute: Some(TransferDSPStateAttribute::OnRequest.to_string()),
                reason: None,
                code: None,
            },
            callback_address: body
                .provider_address
                .as_deref()
                .and_then(|u| Url::parse(u).ok()),
            connector_instance_id: None,
            agreement_id: Urn::from_str(body.agreement_id.as_deref().unwrap_or_default()).map_err(
                |_| Errors::format(BadFormat::Received, "agreementId is not a URN", None),
            )?,
            peer_participant_id: ParticipantId::new(domain.peer_id.clone()),
            identifiers: Some(HashMap::from([
                (CONSUMER_PID_KEY.to_string(), consumer_pid),
                (PROVIDER_PID_KEY.to_string(), provider_pid),
            ])),
            properties: Some(json!({ DIRECTION_PROPERTY: domain.transfer_direction })),
        })
    }
}
