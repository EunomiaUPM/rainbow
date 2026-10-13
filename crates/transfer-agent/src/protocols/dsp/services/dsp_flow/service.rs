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

//! The inbound transfer flow: load with the loader, apply with the message's handler.

use std::sync::Arc;
use common::dsp_common::DspCaller;
use common::flow::{FlowCx, FlowTrait};
use serde_json::Value;
use ymir::errors::{Errors, Outcome};

use crate::protocols::dsp::entities::context_dsp::{
    TransferDspDomain, TransferDspOutcome, TransferDspTyped,
};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::services::dsp_flow::handlers::completion::CompletionHandler;
use crate::protocols::dsp::services::dsp_flow::handlers::request::RequestHandler;
use crate::protocols::dsp::services::dsp_flow::handlers::start::StartHandler;
use crate::protocols::dsp::services::dsp_flow::handlers::suspension::SuspensionHandler;
use crate::protocols::dsp::services::dsp_flow::handlers::termination::TerminationHandler;
use crate::protocols::dsp::services::dsp_flow::loader::TransferDspLoader;
use crate::protocols::dsp::services::dsp_persistence::service::TransferDspPersistence;
use crate::setup::context::AppContext;

/// The five inbound transfer messages.
pub struct TransferDspFlow {
    ctx: Arc<AppContext>,
    loader: TransferDspLoader,
    persistence: TransferDspPersistence,
}

impl TransferDspFlow {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self {
            loader: TransferDspLoader::new(ctx.clone()),
            persistence: TransferDspPersistence::new(ctx.clone()),
            ctx,
        }
    }
}

#[async_trait::async_trait]
impl FlowTrait for TransferDspFlow {
    type Key = TransferDSPMessageType;
    type Caller = DspCaller;
    type Input = TransferDspTyped;
    type Facts = TransferDspDomain;
    type Output = TransferDspOutcome;

    /// The route's message type; the body's own type is checked against it by a shape rule.
    fn key(&self, typed: &TransferDspTyped) -> TransferDSPMessageType {
        typed.route().clone()
    }

    async fn load(
        &self,
        cx: &FlowCx<'_, DspCaller>,
        typed: TransferDspTyped,
    ) -> Outcome<TransferDspDomain> {
        self.loader.load(cx.caller, typed).await
    }

    async fn apply(
        &self,
        _cx: &FlowCx<'_, DspCaller>,
        domain: TransferDspDomain,
        _reply: Option<Value>,
    ) -> Outcome<TransferDspOutcome> {
        use TransferDSPMessageType::*;
        let (ctx, persistence) = (self.ctx.as_ref(), &self.persistence);
        match domain.typed.message {
            TransferRequestMessage => RequestHandler::handle(ctx, persistence, domain).await,
            TransferStartMessage => StartHandler::handle(ctx, persistence, domain).await,
            TransferCompletionMessage => CompletionHandler::handle(ctx, persistence, domain).await,
            TransferSuspensionMessage => SuspensionHandler::handle(ctx, persistence, domain).await,
            TransferTerminationMessage => {
                TerminationHandler::handle(ctx, persistence, domain).await
            }
            other => Err(Errors::crazy(
                format!("{other} is not an inbound message"),
                None,
            )),
        }
    }
}
