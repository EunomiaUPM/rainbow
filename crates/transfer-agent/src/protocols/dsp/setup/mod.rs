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

use std::sync::{Arc, Mutex};

use axum::Router;
use common::boot::workers::BackgroundWorker;
use common::config::types::traits::CommonConfigTrait;
use common::flow::{IdempotencyGuard, InMemoryIdempotencyStore};
use common::flow::{AutocommitUnitOfWorkFactory, FlowRunner};
use common::module_loader::service_module::ServiceModuleTrait;
use ymir::config::traits::HostsConfigTrait;
use ymir::config::types::HostType;

use crate::protocols::dsp::services::dsp_flow::loader::TransferDspLoader;
use crate::protocols::dsp::services::dsp_flow::service::TransferDspFlow;
use crate::protocols::dsp::http::dsp::DspRouter;
use crate::protocols::dsp::http::rpc::TransferRpcRouter;
use crate::protocols::dsp::services::validator::rpc_registry::TransferRpcRules;
use crate::protocols::dsp::services::rpc_flow::service::TransferRpcFlow;
use crate::protocols::dsp::services::rpc_flow::loader::TransferRpcLoader;
use crate::protocols::dsp::services::transfer_writer::service::TransferWriter;
use crate::protocols::dsp::services::validator::TransferValidators;
use crate::protocols::dsp::services::tck::guard::TckGuard;
use crate::protocols::dsp::services::tck::worker::TckWorker;
use crate::protocols::dsp::services::tck::service::TckScenarios;
use crate::setup::context::AppContext;

const DSP_BASE_PATH: &str = "/dsp/current/transfers";

/// DSP transfer: the inbound flow peers call, the outbound flow users call over RPC, and the
/// TCK scenario worker when `DSP_TCK_MAPPING` is set.
pub struct DspModule {
    ctx: Arc<AppContext>,
    inbound: Arc<FlowRunner<TransferDspFlow>>,
    outbound: Arc<FlowRunner<TransferRpcFlow>>,
    tck_worker: Mutex<Option<TckWorker>>,
    tck_mode: bool,
}

impl DspModule {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        let writer = TransferWriter {
            processes: ctx.transfer_process_svc.clone(),
            process_repo: ctx.transfer_process_repo.clone(),
            messages: ctx.transfer_message_svc.clone(),
            dataplane: ctx.dataplane.clone(),
        };
        let tck = TckScenarios::from_env();
        let (tck_jobs, tck_queue) = tokio::sync::mpsc::unbounded_channel();
        let tck_guard = tck.map(|scenarios| TckGuard::new(Arc::new(scenarios), tck_jobs));

        let own_dsp_base = format!(
            "{}/dsp/current",
            ctx.config.common().get_host(HostType::Http)
        );
        let mut outbound = FlowRunner::builder(
            Arc::new(TransferRpcFlow::new(
                TransferRpcLoader::new(
                    ctx.transfer_process_repo.clone(),
                    ctx.dataplane.clone(),
                    own_dsp_base,
                ),
                writer.clone(),
                ctx.grants_facade.clone(),
            )),
            Arc::new(AutocommitUnitOfWorkFactory),
            Arc::new(TransferRpcRules::input_rules()),
            Arc::new(TransferRpcRules::fact_rules()),
        )
        .reply_rules(Arc::new(TransferRpcRules::reply_rules()));
        if let Some(guard) = &tck_guard {
            outbound = outbound.guard(Arc::new(guard.clone()));
        }
        let outbound = Arc::new(outbound.build());

        let loader = TransferDspLoader::new(
            ctx.transfer_process_repo.clone(),
            ctx.negotiation.clone(),
            ctx.catalog.clone(),
        );
        let mut inbound = FlowRunner::builder(
            Arc::new(TransferDspFlow::new(loader, writer)),
            Arc::new(AutocommitUnitOfWorkFactory),
            Arc::new(TransferValidators::input_rules()),
            Arc::new(TransferValidators::fact_rules()),
        )
        .guard(Arc::new(IdempotencyGuard::new(Arc::new(
            InMemoryIdempotencyStore::new(),
        ))));
        if let Some(guard) = &tck_guard {
            inbound = inbound.guard(Arc::new(guard.clone()));
        }

        let tck_mode = tck_guard.is_some();
        let tck_worker = tck_guard.map(|_| TckWorker::new(tck_queue, outbound.clone()));
        Self {
            ctx,
            inbound: Arc::new(inbound.build()),
            outbound,
            tck_worker: Mutex::new(tck_worker),
            tck_mode,
        }
    }
}

impl ServiceModuleTrait for DspModule {
    fn name(&self) -> &'static str {
        "dsp-transfers"
    }

    /// Peers reach the DSP routes with their grant; users reach the RPC routes with OAuth.
    fn http(&self) -> Option<(String, Router)> {
        let dsp = DspRouter::new(
            self.ctx.grants_facade.clone(),
            self.inbound.clone(),
            self.ctx.transfer_process_repo.clone(),
        )
        .router();
        let rpc = TransferRpcRouter::new(self.outbound.clone())
            .router()
            .route_layer(axum::middleware::from_fn_with_state(
                self.ctx.oauth_validator.clone(),
                ymir::http::OauthHttpMiddleware::run,
            ));
        let mut routes = rpc.merge(dsp);
        // The TCK trigger is a test hook without user auth: only served in TCK mode.
        if self.tck_mode {
            routes = routes.merge(TransferRpcRouter::new(self.outbound.clone()).tck_router());
        }
        Some((DSP_BASE_PATH.to_string(), routes))
    }

    fn workers(&self) -> Vec<Box<dyn BackgroundWorker>> {
        self.tck_worker
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take()
            .map(|w| Box::new(w) as Box<dyn BackgroundWorker>)
            .into_iter()
            .collect()
    }
}
