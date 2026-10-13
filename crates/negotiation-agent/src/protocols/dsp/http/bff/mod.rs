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

//! The business routes the GUI calls: each sends one message through the outbound flow, like
//! its RPC route. The provider no longer verifies on the consumer's behalf.

use std::sync::Arc;

use axum::extract::{Request, State};
use axum::response::Response;
use axum::routing::post;
use axum::Router;

use crate::protocols::dsp::entities::message_types::NegotiationRpcMessage;
use crate::protocols::dsp::http::rpc::NegotiationRpcRouter;
use crate::protocols::dsp::setup::context::DspContext;

#[derive(Clone)]
pub struct NegotiationBffRouter {
    rpc: NegotiationRpcRouter,
}

impl NegotiationBffRouter {
    pub fn new(ctx: Arc<DspContext>) -> Self {
        Self {
            rpc: NegotiationRpcRouter::new(ctx),
        }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/bff-rpc/setup-request-init",
                post(Self::handle_request_init),
            )
            .route("/bff-rpc/setup-offer-init", post(Self::handle_offer_init))
            .route("/bff-rpc/setup-acceptance", post(Self::handle_acceptance))
            .route("/bff-rpc/setup-agreement", post(Self::handle_agreement))
            .route("/bff-rpc/setup-termination", post(Self::handle_termination))
            .with_state(self)
    }

    async fn handle_request_init(State(s): State<Self>, request: Request) -> Response {
        s.rpc
            .dispatch(NegotiationRpcMessage::RequestInit, request)
            .await
    }

    async fn handle_offer_init(State(s): State<Self>, request: Request) -> Response {
        s.rpc
            .dispatch(NegotiationRpcMessage::OfferInit, request)
            .await
    }

    async fn handle_acceptance(State(s): State<Self>, request: Request) -> Response {
        s.rpc
            .dispatch(NegotiationRpcMessage::Acceptance, request)
            .await
    }

    /// The provider agrees; verifying is the consumer's, and finalizing follows it.
    async fn handle_agreement(State(s): State<Self>, request: Request) -> Response {
        s.rpc
            .dispatch(NegotiationRpcMessage::Agreement, request)
            .await
    }

    async fn handle_termination(State(s): State<Self>, request: Request) -> Response {
        s.rpc
            .dispatch(NegotiationRpcMessage::Termination, request)
            .await
    }
}
