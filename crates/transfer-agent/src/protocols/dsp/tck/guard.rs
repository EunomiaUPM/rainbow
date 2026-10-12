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

//! Guards that tell the TCK worker what just committed, so a scenario can carry on.

use std::sync::Arc;

use common::facades::grants_facade::VerifiedPeer;
use common::flow::FlowGuardTrait;
use common::oauth::UserInfo;
use tokio::sync::mpsc::UnboundedSender;
use ymir::errors::Outcome;

use crate::entities::protocol::TransferRole;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::flow::{TransferDspFlow, TransferDspReply};
use crate::protocols::dsp::rpc_flow::{TransferRpcFlow, TransferRpcReply};
use crate::protocols::dsp::tck::worker::TckJob;
use crate::protocols::dsp::tck::{TckEvent, TckScenarios};

/// Turns committed changes into scenario jobs for the worker.
#[derive(Clone)]
pub struct TckGuard {
    scenarios: Arc<TckScenarios>,
    jobs: UnboundedSender<TckJob>,
}

impl TckGuard {
    pub fn new(scenarios: Arc<TckScenarios>, jobs: UnboundedSender<TckJob>) -> Self {
        Self { scenarios, jobs }
    }

    fn enqueue(&self, agreement: &str, event: TckEvent, pid: String) {
        let steps = self.scenarios.steps(agreement, event);
        if steps.is_empty() {
            return;
        }
        tracing::info!(agreement, ?event, pid, "TCK scenario continues");
        let _ = self.jobs.send(TckJob { pid, steps });
    }
}

#[async_trait::async_trait]
impl FlowGuardTrait<TransferDspFlow> for TckGuard {
    async fn after_commit(
        &self,
        _peer: &VerifiedPeer,
        key: &TransferDSPMessageType,
        out: &TransferDspReply,
    ) -> Outcome<()> {
        let pid = out.ack.provider_pid.clone();
        match (key, out.role) {
            (TransferDSPMessageType::TransferRequestMessage, TransferRole::Provider) => {
                self.enqueue(&out.agreement_id, TckEvent::ProviderRequested, pid)
            }
            (TransferDSPMessageType::TransferStartMessage, TransferRole::Consumer) => self.enqueue(
                &out.agreement_id,
                TckEvent::ConsumerStarted,
                out.ack.consumer_pid.clone(),
            ),
            _ => {}
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl FlowGuardTrait<TransferRpcFlow> for TckGuard {
    async fn after_commit(
        &self,
        _user: &UserInfo,
        key: &TransferDSPMessageType,
        out: &TransferRpcReply,
    ) -> Outcome<()> {
        if *key == TransferDSPMessageType::TransferRequestMessage
            && out.role == TransferRole::Consumer
        {
            let pid = out
                .transfer_agent_model
                .correlation
                .consumer_pid
                .clone()
                .unwrap_or_default();
            self.enqueue(&out.agreement_id, TckEvent::ConsumerRequested, pid);
        }
        Ok(())
    }
}
