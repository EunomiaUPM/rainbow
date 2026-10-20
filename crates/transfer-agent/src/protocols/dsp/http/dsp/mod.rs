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

//! DSP transfer routes (DSP 10.2): every message goes through the inbound flow runner.

pub mod adapter;

use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, Request, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Extension, Json, Router};
use common::dsp_common::persistence::DspPersistenceTrait;
use common::dsp_common::{DspCaller, PeerAuth};
use http::StatusCode;

use crate::protocols::dsp::entities::ack::TransferProcessAck;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::entities::transfer_error::TransferErrorDto;
use crate::protocols::dsp::http::dsp::adapter::TransferDspAdapter;
use crate::protocols::dsp::setup::context::DspContext;
use common::routes::transfer as routes;

#[derive(Clone)]
pub struct DspRouter {
    ctx: Arc<DspContext>,
}

impl DspRouter {
    pub fn new(ctx: Arc<DspContext>) -> Self {
        Self { ctx }
    }

    /// Every route behind `PeerAuth`: an unknown peer gets the 404 of a missing process.
    pub fn router(self) -> Router {
        let auth = &self.ctx.app.ports.auth;
        let peer_auth = PeerAuth::new(auth.grants.clone(), auth.mates.clone());
        Router::new()
            .route(routes::dsp::REQUEST, post(Self::handle_transfer_request))
            .route(routes::dsp::PROCESS, get(Self::handle_get_transfer_process))
            .route(routes::dsp::PROCESS_START, post(Self::handle_transfer_start))
            .route(routes::dsp::PROCESS_COMPLETION, post(Self::handle_transfer_completion))
            .route(routes::dsp::PROCESS_TERMINATION, post(Self::handle_transfer_termination))
            .route(routes::dsp::PROCESS_SUSPENSION, post(Self::handle_transfer_suspension))
            .layer(middleware::from_fn_with_state(peer_auth, PeerAuth::run))
            .with_state(self)
    }

    async fn dispatch(&self, key: TransferDSPMessageType, request: Request) -> Response {
        self.ctx
            .inbound
            .serve(&TransferDspAdapter, key, request)
            .await
    }

    async fn handle_transfer_request(State(state): State<DspRouter>, request: Request) -> Response {
        state
            .dispatch(TransferDSPMessageType::TransferRequestMessage, request)
            .await
    }

    async fn handle_transfer_start(State(state): State<DspRouter>, request: Request) -> Response {
        state
            .dispatch(TransferDSPMessageType::TransferStartMessage, request)
            .await
    }

    async fn handle_transfer_completion(
        State(state): State<DspRouter>,
        request: Request,
    ) -> Response {
        state
            .dispatch(TransferDSPMessageType::TransferCompletionMessage, request)
            .await
    }

    async fn handle_transfer_termination(
        State(state): State<DspRouter>,
        request: Request,
    ) -> Response {
        state
            .dispatch(TransferDSPMessageType::TransferTerminationMessage, request)
            .await
    }

    async fn handle_transfer_suspension(
        State(state): State<DspRouter>,
        request: Request,
    ) -> Response {
        state
            .dispatch(TransferDSPMessageType::TransferSuspensionMessage, request)
            .await
    }

    /// DSP 10.2.1: the process as the peer knows it; unknown and foreign answer alike (404).
    async fn handle_get_transfer_process(
        State(state): State<DspRouter>,
        Extension(caller): Extension<DspCaller>,
        Path(id): Path<String>,
    ) -> Response {
        match state.find_for(&caller, &id).await {
            Some(ack) => (StatusCode::OK, Json(ack)).into_response(),
            None => {
                let body = TransferErrorDto::new(
                    id.clone(),
                    id,
                    "404".into(),
                    vec!["transfer process not found".into()],
                );
                (StatusCode::NOT_FOUND, Json(body)).into_response()
            }
        }
    }

    async fn find_for(&self, caller: &DspCaller, pid: &str) -> Option<TransferProcessAck> {
        let process = self.ctx.persistence.hydrate_process(pid).await.ok()??;
        let correlation = process.correlation();
        let counterparty = correlation
            .peer_participant_id
            .as_ref()
            .map(|p| p.as_str())
            .unwrap_or_default();
        caller
            .actor()
            .authorize(process.owner(), counterparty, process.id().as_urn())
            .ok()?;
        Some(TransferProcessAck::new(
            correlation.consumer_pid.clone().unwrap_or_default(),
            correlation.provider_pid.clone().unwrap_or_default(),
            TransferDSPState::from_str(process.state().0.as_str()).ok()?,
        ))
    }
}
