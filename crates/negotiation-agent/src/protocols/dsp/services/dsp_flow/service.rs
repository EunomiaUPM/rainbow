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

//! The inbound negotiation flow: load with the loader, apply with the message's handler.

use std::sync::Arc;

use common::dsp_common::DspCaller;
use common::flow::{FlowCx, FlowTrait};
use serde_json::Value;
use ymir::errors::{Errors, Outcome};

use crate::protocols::dsp::entities::context_dsp::{
    NegotiationDspDomain, NegotiationDspOutcome, NegotiationDspTyped,
};
use crate::protocols::dsp::entities::message_types::NegotiationMessageType;
use crate::protocols::dsp::services::dsp_flow::handlers::agreement::AgreementHandler;
use crate::protocols::dsp::services::dsp_flow::handlers::event::EventHandler;
use crate::protocols::dsp::services::dsp_flow::handlers::offer::OfferHandler;
use crate::protocols::dsp::services::dsp_flow::handlers::request::RequestHandler;
use crate::protocols::dsp::services::dsp_flow::handlers::termination::TerminationHandler;
use crate::protocols::dsp::services::dsp_flow::handlers::verification::VerificationHandler;
use crate::protocols::dsp::services::dsp_flow::loader::NegotiationDspLoader;
use crate::protocols::dsp::services::dsp_persistence::service::NegotiationDspPersistence;
use crate::setup::context::AppContext;

/// The seven inbound negotiation messages.
pub struct NegotiationDspFlow {
    loader: NegotiationDspLoader,
    persistence: NegotiationDspPersistence,
}

impl NegotiationDspFlow {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self {
            loader: NegotiationDspLoader::new(ctx.clone()),
            persistence: NegotiationDspPersistence::new(ctx),
        }
    }
}

#[async_trait::async_trait]
impl FlowTrait for NegotiationDspFlow {
    type Key = NegotiationMessageType;
    type Caller = DspCaller;
    type Input = NegotiationDspTyped;
    type Facts = NegotiationDspDomain;
    type Output = NegotiationDspOutcome;

    /// The body's type when it is one the route takes (the events route takes both events),
    /// the route's otherwise; the mismatch is a shape rule.
    fn key(&self, typed: &NegotiationDspTyped) -> NegotiationMessageType {
        if typed.message.wire_type() == typed.route().wire_type() {
            typed.message
        } else {
            typed.route()
        }
    }

    async fn load(
        &self,
        cx: &FlowCx<'_, DspCaller>,
        typed: NegotiationDspTyped,
    ) -> Outcome<NegotiationDspDomain> {
        self.loader.load(cx.caller, typed).await
    }

    async fn apply(
        &self,
        _cx: &FlowCx<'_, DspCaller>,
        domain: NegotiationDspDomain,
        _reply: Option<Value>,
    ) -> Outcome<NegotiationDspOutcome> {
        use NegotiationMessageType::*;
        let persistence = &self.persistence;
        match domain.typed.message {
            ContractRequestMessage => RequestHandler::handle(persistence, domain).await,
            ContractOfferMessage => OfferHandler::handle(persistence, domain).await,
            EventAccepted | EventFinalized => EventHandler::handle(persistence, domain).await,
            ContractAgreementMessage => AgreementHandler::handle(persistence, domain).await,
            ContractAgreementVerificationMessage => {
                VerificationHandler::handle(persistence, domain).await
            }
            ContractNegotiationTerminationMessage => {
                TerminationHandler::handle(persistence, domain).await
            }
            other => Err(Errors::crazy(
                format!("{other} is not an inbound message"),
                None,
            )),
        }
    }
}
