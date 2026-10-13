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

pub mod agreement;
pub mod event;
pub mod offer;
pub mod request;
pub mod termination;
pub mod verification;

use std::collections::HashMap;

use common::dsp_common::persistence::StateChange;
use ymir::errors::{Errors, Outcome};

use crate::entities::negotiation_process::NewNegotiationProcessDto;
use crate::protocols::dsp::entities::context_common::NegotiationProcessSlot;
use crate::protocols::dsp::entities::context_dsp::{NegotiationDspDomain, NegotiationDspOutcome};
use crate::protocols::dsp::entities::process::NegotiationProcess;
use crate::protocols::dsp::entities::role::NegotiationRole;
use crate::protocols::dsp::entities::state::NegotiationState;
use crate::protocols::dsp::entities::state_change::{
    MessageDirection, NegotiationAttachment, NegotiationMessageRecord, OfferRecord,
};
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;

/// The state change a peer's message causes on an existing process.
pub struct InboundTransition;

impl InboundTransition {
    /// Moves the process to `target`, keeps the message and what it brought.
    pub async fn apply(
        persistence: &NegotiationDspPersistence,
        domain: &NegotiationDspDomain,
        target: NegotiationState,
        attachment: NegotiationAttachment,
    ) -> Outcome<NegotiationDspOutcome> {
        let fields = &domain.typed.fields;
        let change = StateChange {
            target,
            code: fields.code.clone(),
            reason: (!fields.reason.is_empty()).then(|| fields.reason.clone()),
            message: Self::record(domain),
        };
        let process = persistence
            .change_with(domain.existing()?, change, attachment)
            .await?;
        Ok(NegotiationDspOutcome::applied(domain, &process, false))
    }

    /// Opens the process a request or offer starts, minting our pid, with the offer it brought.
    pub async fn open(
        persistence: &NegotiationDspPersistence,
        domain: &NegotiationDspDomain,
        state: NegotiationState,
    ) -> Outcome<NegotiationDspOutcome> {
        let NegotiationProcessSlot::New { peer_pid } = &domain.process else {
            return Err(Errors::crazy("only a new process is opened", None));
        };
        let own_pid = format!("urn:uuid:{}", uuid::Uuid::new_v4());
        let (consumer_pid, provider_pid) = match domain.role {
            NegotiationRole::Provider => (peer_pid, &own_pid),
            NegotiationRole::Consumer => (&own_pid, peer_pid),
        };
        let cmd = NewNegotiationProcessDto {
            id: None,
            visibility: None,
            owner: Some(domain.owner()),
            state: state.to_string(),
            state_attribute: None,
            associated_agent_peer: domain.typed.peer().participant_id.clone(),
            protocol: NegotiationDspPersistence::PROTOCOL.to_string(),
            callback_address: domain.typed.fields.callback_address.clone(),
            role: domain.role.to_string(),
            properties: None,
            identifiers: Some(HashMap::from([
                (
                    NegotiationProcess::CONSUMER_PID.to_string(),
                    consumer_pid.clone(),
                ),
                (
                    NegotiationProcess::PROVIDER_PID.to_string(),
                    provider_pid.clone(),
                ),
            ])),
        };
        let process = persistence
            .open_process(
                &NegotiationDspPersistence::actor(),
                &cmd,
                Self::record(domain),
                Self::offer(domain)?,
            )
            .await?;
        Ok(NegotiationDspOutcome::applied(domain, &process, true))
    }

    /// The offer a request or offer message brought, as stored.
    pub fn offer(domain: &NegotiationDspDomain) -> Outcome<NegotiationAttachment> {
        let offer_id = domain
            .typed
            .fields
            .offer
            .as_ref()
            .and_then(|o| o.id.clone())
            .ok_or_else(|| {
                Errors::crazy("an offer message without an offer passed the rules", None)
            })?;
        Ok(NegotiationAttachment::Offer(OfferRecord {
            offer_id,
            content: domain.typed.offer_content().cloned().unwrap_or_default(),
        }))
    }

    fn record(domain: &NegotiationDspDomain) -> NegotiationMessageRecord {
        NegotiationMessageRecord {
            message: domain.typed.message,
            direction: MessageDirection::Inbound,
            payload: domain.typed.json().clone(),
        }
    }
}
