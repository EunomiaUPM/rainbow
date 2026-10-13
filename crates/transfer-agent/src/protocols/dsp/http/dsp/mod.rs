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

//! DSP transfer routes (DSP 10.2): every message goes through the transfer flow runner.

pub mod adapter;

use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Extension, Json, Router};
use common::dsp_common::DspActor;
use common::facades::grants_facade::{GrantsFacadeTrait, VerifiedPeer};
use common::flow::FlowRunner;
use common::oauth::OwnerScope;
use http::StatusCode;
use ymir::http::OauthHttpMiddleware;

use crate::data::repo::transfer_process::TransferProcessRepoTrait;
use crate::protocols::dsp::entities::ack::TransferProcessAck;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::state::TransferDSPState;
use crate::protocols::dsp::entities::transfer_error::TransferErrorDto;
use crate::protocols::dsp::http::dsp::adapter::TransferDspAdapter;
use crate::protocols::dsp::services::dsp_flow::service::TransferDspFlow;

#[derive(Clone)]
pub struct DspRouter {
    grants: Arc<dyn GrantsFacadeTrait>,
    runner: Arc<FlowRunner<TransferDspFlow>>,
    processes: Arc<dyn TransferProcessRepoTrait>,
}

impl DspRouter {
    pub fn new(
        grants: Arc<dyn GrantsFacadeTrait>,
        runner: Arc<FlowRunner<TransferDspFlow>>,
        processes: Arc<dyn TransferProcessRepoTrait>,
    ) -> Self {
        Self {
            grants,
            runner,
            processes,
        }
    }

    async fn auth_middleware(
        State(state): State<DspRouter>,
        mut request: Request,
        next: Next,
    ) -> Result<impl IntoResponse, StatusCode> {
        // DSP 10.1.2.3: an unauthorized client gets a 404 like a missing process (10.1.2.2),
        // so probing cannot reveal which Transfer Processes exist.
        let token = OauthHttpMiddleware::bearer(request.headers())
            .map_err(|_| StatusCode::NOT_FOUND)?
            .to_owned();
        match state.grants.verify_token(token).await {
            Ok(peer) => {
                request.extensions_mut().insert(peer);
                Ok(next.run(request).await)
            }
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/request", post(Self::handle_transfer_request))
            .route("/{id}", get(Self::handle_get_transfer_process))
            .route("/{id}/start", post(Self::handle_transfer_start))
            .route("/{id}/completion", post(Self::handle_transfer_completion))
            .route("/{id}/termination", post(Self::handle_transfer_termination))
            .route("/{id}/suspension", post(Self::handle_transfer_suspension))
            .layer(middleware::from_fn_with_state(
                self.clone(),
                Self::auth_middleware,
            ))
            .with_state(self)
    }

    /// Decode, run and encode: the same three steps for every message route.
    async fn dispatch(
        &self,
        peer: &VerifiedPeer,
        key: TransferDSPMessageType,
        request: Request,
    ) -> Response {
        self.runner
            .serve(&TransferDspAdapter, peer, key, request)
            .await
    }

    async fn handle_transfer_request(
        State(state): State<DspRouter>,
        Extension(peer): Extension<VerifiedPeer>,
        request: Request,
    ) -> Response {
        state
            .dispatch(
                &peer,
                TransferDSPMessageType::TransferRequestMessage,
                request,
            )
            .await
    }

    async fn handle_transfer_start(
        State(state): State<DspRouter>,
        Extension(peer): Extension<VerifiedPeer>,
        request: Request,
    ) -> Response {
        state
            .dispatch(&peer, TransferDSPMessageType::TransferStartMessage, request)
            .await
    }

    async fn handle_transfer_completion(
        State(state): State<DspRouter>,
        Extension(peer): Extension<VerifiedPeer>,
        request: Request,
    ) -> Response {
        state
            .dispatch(
                &peer,
                TransferDSPMessageType::TransferCompletionMessage,
                request,
            )
            .await
    }

    async fn handle_transfer_termination(
        State(state): State<DspRouter>,
        Extension(peer): Extension<VerifiedPeer>,
        request: Request,
    ) -> Response {
        state
            .dispatch(
                &peer,
                TransferDSPMessageType::TransferTerminationMessage,
                request,
            )
            .await
    }

    async fn handle_transfer_suspension(
        State(state): State<DspRouter>,
        Extension(peer): Extension<VerifiedPeer>,
        request: Request,
    ) -> Response {
        state
            .dispatch(
                &peer,
                TransferDSPMessageType::TransferSuspensionMessage,
                request,
            )
            .await
    }

    /// DSP 10.2.1: the process as the peer knows it; unknown and foreign answer alike (404).
    async fn handle_get_transfer_process(
        State(state): State<DspRouter>,
        Extension(peer): Extension<VerifiedPeer>,
        Path(id): Path<String>,
    ) -> Response {
        match state.find_for_peer(&peer, &id).await {
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

    async fn find_for_peer(&self, peer: &VerifiedPeer, pid: &str) -> Option<TransferProcessAck> {
        let process = self
            .processes
            .get_transfer_process_by_key_value(&OwnerScope::All, pid)
            .await
            .ok()??;
        let correlation = process.correlation();
        let counterparty = correlation
            .peer_participant_id
            .as_ref()
            .map(|p| p.as_str())
            .unwrap_or_default();
        DspActor::peer(peer)
            .authorize(process.owner(), counterparty, process.id().as_urn())
            .ok()?;
        Some(TransferProcessAck::new(
            correlation.consumer_pid.clone().unwrap_or_default(),
            correlation.provider_pid.clone().unwrap_or_default(),
            TransferDSPState::from_str(process.state().0.as_str()).ok()?,
        ))
    }
}
