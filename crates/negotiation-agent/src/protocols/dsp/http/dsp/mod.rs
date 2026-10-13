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

//! DSP negotiation routes (DSP 8.2, 8.3): every message goes through the inbound flow runner.

pub mod adapter;

use std::sync::Arc;

use axum::extract::{Path, Request, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Extension, Json, Router};
use common::dsp_common::persistence::DspPersistenceTrait;
use common::dsp_common::{DspCaller, PeerAuth};
use http::StatusCode;

use crate::protocols::dsp::entities::ack::NegotiationAck;
use crate::protocols::dsp::entities::message_types::NegotiationMessageType;
use crate::protocols::dsp::entities::negotiation_error::NegotiationErrorDto;
use crate::protocols::dsp::http::dsp::adapter::NegotiationDspAdapter;
use crate::protocols::dsp::setup::context::DspContext;

#[derive(Clone)]
pub struct DspRouter {
    ctx: Arc<DspContext>,
}

impl DspRouter {
    pub fn new(ctx: Arc<DspContext>) -> Self {
        Self { ctx }
    }

    /// Every route behind `PeerAuth`: an unknown peer gets the 404 of a missing negotiation.
    pub fn router(self) -> Router {
        let auth = &self.ctx.app.ports.auth;
        let peer_auth = PeerAuth::new(auth.grants.clone(), auth.mates.clone());
        Router::new()
            .route("/{id}", get(Self::handle_get_negotiation))
            .route("/request", post(Self::handle_request))
            .route("/{id}/request", post(Self::handle_request))
            .route("/offers", post(Self::handle_offer))
            .route("/{id}/offers", post(Self::handle_offer))
            .route("/{id}/events", post(Self::handle_event))
            .route("/{id}/agreement", post(Self::handle_agreement))
            .route(
                "/{id}/agreement/verification",
                post(Self::handle_verification),
            )
            .route("/{id}/termination", post(Self::handle_termination))
            .layer(middleware::from_fn_with_state(peer_auth, PeerAuth::run))
            .with_state(self)
    }

    async fn dispatch(&self, key: NegotiationMessageType, request: Request) -> Response {
        self.ctx
            .inbound
            .serve(&NegotiationDspAdapter, key, request)
            .await
    }

    /// DSP 8.2.2, 8.2.3: a request opening a negotiation, or countering an offer.
    async fn handle_request(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationMessageType::ContractRequestMessage, request)
            .await
    }

    /// DSP 8.3.3, 8.3.4: an offer opening a negotiation, or countering a request.
    async fn handle_offer(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationMessageType::ContractOfferMessage, request)
            .await
    }

    /// DSP 8.2.4, 8.3.6: either event; the body's `eventType` picks which.
    async fn handle_event(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationMessageType::EventAccepted, request)
            .await
    }

    /// DSP 8.3.5.
    async fn handle_agreement(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationMessageType::ContractAgreementMessage, request)
            .await
    }

    /// DSP 8.2.5.
    async fn handle_verification(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(
            NegotiationMessageType::ContractAgreementVerificationMessage,
            request,
        )
        .await
    }

    /// DSP 8.2.6, 8.3.7.
    async fn handle_termination(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(
            NegotiationMessageType::ContractNegotiationTerminationMessage,
            request,
        )
        .await
    }

    /// DSP 8.2.1, 8.3.2: the negotiation as the peer knows it; unknown and foreign answer alike.
    async fn handle_get_negotiation(
        State(state): State<Self>,
        Extension(caller): Extension<DspCaller>,
        Path(id): Path<String>,
    ) -> Response {
        match state.find_for(&caller, &id).await {
            Some(ack) => (StatusCode::OK, Json(ack)).into_response(),
            None => {
                let body = NegotiationErrorDto::new(
                    id.clone(),
                    id,
                    "404".into(),
                    vec!["negotiation not found".into()],
                );
                (StatusCode::NOT_FOUND, Json(body)).into_response()
            }
        }
    }

    async fn find_for(&self, caller: &DspCaller, pid: &str) -> Option<NegotiationAck> {
        let process = self.ctx.persistence.hydrate_process(pid).await.ok()??;
        caller
            .actor()
            .authorize(&process.owner(), process.peer_id(), &process.id().ok()?)
            .ok()?;
        Some(NegotiationAck::new(
            process.consumer_pid.unwrap_or_default(),
            process.provider_pid.unwrap_or_default(),
            process.state,
        ))
    }
}
