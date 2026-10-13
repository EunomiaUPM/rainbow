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

use crate::entities::protocol::TransferRole;
use crate::protocols::dsp::entities::context_dsp::TransferDspOutcome;
use crate::protocols::dsp::entities::context_rpc::TransferRpcOutcome;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::services::dsp_flow::service::TransferDspFlow;
use crate::protocols::dsp::services::rpc_flow::service::TransferRpcFlow;
use crate::protocols::dsp::services::tck::service::TransferTckScript;
use crate::protocols::dsp::entities::tck::TckEvent;

/// Turns committed changes of both flows into TCK events.
#[derive(Clone)]
pub struct TckGuard {
    queue: TckQueue<TransferTckScript>,
}

impl TckGuard {
    pub fn new(queue: TckQueue<TransferTckScript>) -> Self {
        Self { queue }
    }
}

#[async_trait::async_trait]
impl FlowGuardTrait<TransferDspFlow> for TckGuard {
    async fn after_commit(
        &self,
        _caller: &DspCaller,
        key: &TransferDSPMessageType,
        out: &TransferDspOutcome,
    ) -> Outcome<()> {
        let (agreement, ack) = (&out.agreement_id, &out.ack);
        match (key, out.role) {
            (TransferDSPMessageType::TransferRequestMessage, TransferRole::Provider) => {
                self.queue.enqueue(
                    agreement,
                    TckEvent::ProviderRequested,
                    ack.provider_pid.clone(),
                )
            }
            (TransferDSPMessageType::TransferStartMessage, TransferRole::Consumer) => {
                self.queue.enqueue(
                    agreement,
                    TckEvent::ConsumerStarted,
                    ack.consumer_pid.clone(),
                )
            }
            _ => {}
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl FlowGuardTrait<TransferRpcFlow> for TckGuard {
    async fn after_commit(
        &self,
        _caller: &DspCaller,
        key: &TransferDSPMessageType,
        out: &TransferRpcOutcome,
    ) -> Outcome<()> {
        if *key == TransferDSPMessageType::TransferRequestMessage
            && out.role == TransferRole::Consumer
        {
            let pid = out
                .process
                .correlation()
                .consumer_pid
                .clone()
                .unwrap_or_default();
            self.queue
                .enqueue(&out.agreement_id, TckEvent::ConsumerRequested, pid);
        }
        Ok(())
    }
}
