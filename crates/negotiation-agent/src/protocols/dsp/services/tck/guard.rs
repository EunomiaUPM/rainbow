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

//! The guard that tells the TCK queue what just committed, so a scenario can carry on.

use common::dsp_common::DspCaller;
use common::flow::FlowGuardTrait;
use common::tck::TckQueue;
use ymir::errors::Outcome;

use crate::protocols::dsp::entities::context_dsp::NegotiationDspOutcome;
use crate::protocols::dsp::entities::context_rpc::NegotiationRpcOutcome;
use crate::protocols::dsp::entities::message_types::{
    NegotiationMessageType, NegotiationRpcMessage,
};
use crate::protocols::dsp::entities::role::NegotiationRole;
use crate::protocols::dsp::entities::tck::TckEvent;
use crate::protocols::dsp::services::dsp_flow::service::NegotiationDspFlow;
use crate::protocols::dsp::services::rpc_flow::service::NegotiationRpcFlow;
use crate::protocols::dsp::services::tck::service::NegotiationTckScript;

/// Turns committed changes of both flows into TCK events.
#[derive(Clone)]
pub struct TckGuard {
    queue: TckQueue<NegotiationTckScript>,
}

impl TckGuard {
    pub fn new(queue: TckQueue<NegotiationTckScript>) -> Self {
        Self { queue }
    }

    /// What a peer's message, applied on our side of a negotiation, means for a scenario.
    fn inbound_event(out: &NegotiationDspOutcome, key: NegotiationMessageType) -> Option<TckEvent> {
        use NegotiationMessageType::*;
        Some(match (out.role, key) {
            (NegotiationRole::Provider, ContractRequestMessage) if out.created => {
                TckEvent::ProviderRequested
            }
            (NegotiationRole::Provider, ContractRequestMessage) => {
                TckEvent::ProviderCounterRequested
            }
            (NegotiationRole::Provider, EventAccepted) => TckEvent::ProviderAccepted,
            (NegotiationRole::Provider, ContractAgreementVerificationMessage) => {
                TckEvent::ProviderVerified
            }
            (NegotiationRole::Consumer, ContractOfferMessage) => TckEvent::ConsumerOffered,
            (NegotiationRole::Consumer, ContractAgreementMessage) => TckEvent::ConsumerAgreed,
            _ => return None,
        })
    }
}

#[async_trait::async_trait]
impl FlowGuardTrait<NegotiationDspFlow> for TckGuard {
    async fn after_commit(
        &self,
        _caller: &DspCaller,
        key: &NegotiationMessageType,
        out: &NegotiationDspOutcome,
    ) -> Outcome<()> {
        let (Some(event), Some(dataset)) = (Self::inbound_event(out, *key), &out.opening_target)
        else {
            return Ok(());
        };
        let pid = match out.role {
            NegotiationRole::Provider => out.ack.provider_pid.clone(),
            NegotiationRole::Consumer => out.ack.consumer_pid.clone(),
        };
        self.queue.enqueue(dataset, event, pid);
        Ok(())
    }
}

#[async_trait::async_trait]
impl FlowGuardTrait<NegotiationRpcFlow> for TckGuard {
    async fn after_commit(
        &self,
        _caller: &DspCaller,
        key: &NegotiationRpcMessage,
        out: &NegotiationRpcOutcome,
    ) -> Outcome<()> {
        if let (NegotiationRpcMessage::RequestInit, Some(dataset)) = (key, &out.opening_target) {
            let pid = out.process.consumer_pid.clone().unwrap_or_default();
            self.queue
                .enqueue(dataset, TckEvent::ConsumerRequested, pid);
        }
        Ok(())
    }
}
