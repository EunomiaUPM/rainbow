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

pub mod agreement;
pub mod event;
pub mod offer;
pub mod request;
pub mod termination;
pub mod verification;

use std::collections::HashMap;

use common::dsp_common::persistence::StateChange;
use ymir::types::oauth::UserInfo;
use serde_json::Value;
use ymir::errors::{Errors, Outcome};

use crate::entities::negotiation_process::NewNegotiationProcessDto;
use crate::protocols::dsp::entities::context_rpc::{NegotiationRpcDomain, NegotiationRpcOutcome};
use crate::protocols::dsp::entities::process::NegotiationProcess;
use crate::protocols::dsp::entities::role::NegotiationRole;
use crate::protocols::dsp::entities::state::NegotiationState;
use crate::protocols::dsp::entities::state_change::{
    MessageDirection, NegotiationAttachment, NegotiationMessageRecord, OfferRecord,
};
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;
use crate::setup::context::AppContext;

/// The state change our own message causes, and the process it opens.
pub struct OutboundTransition;

impl OutboundTransition {
    /// Moves the process to `target` and keeps the message as sent, with what it brought.
    pub async fn apply(
        ctx: &AppContext,
        persistence: &NegotiationDspPersistence,
        domain: NegotiationRpcDomain,
        reply: &Value,
        target: NegotiationState,
        attachment: NegotiationAttachment,
    ) -> Outcome<NegotiationRpcOutcome> {
        let change = StateChange {
            target,
            code: domain.typed.body.code.clone(),
            reason: domain.typed.body.reason.clone(),
            message: Self::record(&domain),
        };
        let process = persistence
            .change_with(domain.existing()?, change, attachment)
            .await?;
        Self::sent(ctx, domain, reply, process).await
    }

    /// Stores the process the peer accepted, with both pids, the message and its offer.
    pub async fn open(
        ctx: &AppContext,
        persistence: &NegotiationDspPersistence,
        user: &UserInfo,
        domain: NegotiationRpcDomain,
        reply: &Value,
        state: NegotiationState,
    ) -> Outcome<NegotiationRpcOutcome> {
        let (ours, theirs) = match domain.role {
            NegotiationRole::Consumer => ("consumerPid", "providerPid"),
            NegotiationRole::Provider => ("providerPid", "consumerPid"),
        };
        let fields = serde_json::to_value(&domain.outgoing.fields)?;
        let identifiers = HashMap::from([
            (ours.to_string(), Self::text(&fields[ours])),
            (theirs.to_string(), Self::text(&reply["fields"][theirs])),
        ]);
        let cmd = NewNegotiationProcessDto {
            id: None,
            visibility: None,
            owner: Some(domain.owner.clone()),
            state: state.to_string(),
            state_attribute: None,
            associated_agent_peer: domain.peer_id.clone(),
            protocol: NegotiationDspPersistence::PROTOCOL.to_string(),
            callback_address: Some(domain.peer_address.clone()),
            role: domain.role.to_string(),
            properties: None,
            identifiers: Some(identifiers),
        };
        let process = persistence
            .open_process(user, &cmd, Self::record(&domain), Self::offer(&domain)?)
            .await?;
        Self::sent(ctx, domain, reply, process).await
    }

    /// The offer a request or offer carried, as sent.
    pub fn offer(domain: &NegotiationRpcDomain) -> Outcome<NegotiationAttachment> {
        let offer_id = domain
            .outgoing
            .fields
            .offer
            .as_ref()
            .and_then(|o| o.id.clone())
            .ok_or_else(|| {
                Errors::crazy("an offer message without an offer passed the rules", None)
            })?;
        Ok(NegotiationAttachment::Offer(OfferRecord {
            offer_id,
            content: domain
                .outgoing
                .json
                .get("offer")
                .cloned()
                .unwrap_or_default(),
        }))
    }

    fn record(domain: &NegotiationRpcDomain) -> NegotiationMessageRecord {
        NegotiationMessageRecord {
            message: domain.outgoing.message,
            direction: MessageDirection::Outbound,
            payload: domain.outgoing.json.clone(),
        }
    }

    /// The outcome, with the process as the management API shows it.
    async fn sent(
        ctx: &AppContext,
        domain: NegotiationRpcDomain,
        reply: &Value,
        process: NegotiationProcess,
    ) -> Outcome<NegotiationRpcOutcome> {
        let view = ctx
            .processes()
            .get_one(&NegotiationDspPersistence::actor(), &process.id()?)
            .await?;
        Ok(NegotiationRpcOutcome::sent(domain, reply, process, view))
    }

    fn text(value: &Value) -> String {
        value.as_str().unwrap_or_default().to_string()
    }
}
