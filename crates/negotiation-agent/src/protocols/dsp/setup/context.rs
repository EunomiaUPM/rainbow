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

//! What the DSP plane uses: the agent context, the runners of both flows built on it, and the
//! TCK queue in TCK mode.

use std::sync::Arc;

use common::flow::{AutocommitUnitOfWorkFactory, FlowRunner};
use common::flow::{IdempotencyGuard, InMemoryIdempotencyStore};
use common::tck::{TckMapping, TckQueue, TckWorker};

use crate::protocols::dsp::entities::tck::TckMessage;
use crate::protocols::dsp::services::dsp_flow::service::NegotiationDspFlow;
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;
use crate::protocols::dsp::services::rpc_flow::service::NegotiationRpcFlow;
use crate::protocols::dsp::services::tck::guard::TckGuard;
use crate::protocols::dsp::services::tck::player::NegotiationTckPlayer;
use crate::protocols::dsp::services::tck::service::NegotiationTckScript;
use crate::protocols::dsp::services::validator::{NegotiationDspRules, NegotiationRpcRules};
use crate::setup::context::AppContext;

/// The agent context plus the runners peers and users reach through the DSP routes.
pub struct DspContext {
    pub app: Arc<AppContext>,
    pub persistence: NegotiationDspPersistence,
    pub inbound: Arc<FlowRunner<NegotiationDspFlow>>,
    pub outbound: Arc<FlowRunner<NegotiationRpcFlow>>,
    /// Set when `DSP_TCK_MAPPING` is.
    pub tck: Option<TckQueue<NegotiationTckScript>>,
}

impl DspContext {
    /// The runners, with the TCK guard on both and the worker that plays it in TCK mode.
    pub fn build(app: Arc<AppContext>) -> (Arc<Self>, Option<TckWorker<TckMessage>>) {
        let (queue, jobs) = TckMapping::from_env()
            .map(TckQueue::<NegotiationTckScript>::open)
            .unzip();
        let guard = queue.clone().map(TckGuard::new);
        let outbound = Self::outbound(&app, guard.as_ref());
        let inbound = Self::inbound(&app, guard.as_ref());
        let persistence = NegotiationDspPersistence::new(app.clone());
        let worker = jobs.map(|jobs| {
            let player = NegotiationTckPlayer::new(outbound.clone(), persistence.clone());
            TckWorker::new("negotiation-tck-scenarios", jobs, Arc::new(player))
        });
        let ctx = Self {
            app,
            persistence,
            inbound,
            outbound,
            tck: queue,
        };
        (Arc::new(ctx), worker)
    }

    pub fn tck_mode(&self) -> bool {
        self.tck.is_some()
    }

    /// Peers' messages: replayed when repeated, then seen by the TCK guard.
    fn inbound(
        app: &Arc<AppContext>,
        tck: Option<&TckGuard>,
    ) -> Arc<FlowRunner<NegotiationDspFlow>> {
        let mut runner = FlowRunner::builder(
            Arc::new(NegotiationDspFlow::new(app.clone())),
            Arc::new(AutocommitUnitOfWorkFactory),
            Arc::new(NegotiationDspRules::input_rules()),
            Arc::new(NegotiationDspRules::fact_rules()),
        )
        .guard(Arc::new(IdempotencyGuard::new(Arc::new(
            InMemoryIdempotencyStore::new(),
        ))));
        if let Some(guard) = tck {
            runner = runner.guard(Arc::new(guard.clone()));
        }
        Arc::new(runner.build())
    }

    /// Our own messages, with the peer's answer checked.
    fn outbound(
        app: &Arc<AppContext>,
        tck: Option<&TckGuard>,
    ) -> Arc<FlowRunner<NegotiationRpcFlow>> {
        let mut runner = FlowRunner::builder(
            Arc::new(NegotiationRpcFlow::new(app.clone())),
            Arc::new(AutocommitUnitOfWorkFactory),
            Arc::new(NegotiationRpcRules::input_rules()),
            Arc::new(NegotiationRpcRules::fact_rules()),
        )
        .reply_rules(Arc::new(NegotiationRpcRules::reply_rules()));
        if let Some(guard) = tck {
            runner = runner.guard(Arc::new(guard.clone()));
        }
        Arc::new(runner.build())
    }
}
