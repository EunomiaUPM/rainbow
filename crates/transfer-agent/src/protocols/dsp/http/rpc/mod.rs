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

//! Eunomia RPC routes a local user calls to make the agent send transfer messages, and the endpoint
//! the DSP TCK uses to make a consumer open a transfer (TCK mode only).

pub mod adapter;

use std::sync::Arc;
use axum::body::Body;

use axum::extract::rejection::JsonRejection;
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use common::dsp_common::{paths, DspCaller};
use common::oauth::UserInfo;
use http::header::HOST;
use http::{HeaderMap, StatusCode};
use serde::Deserialize;

use crate::protocols::dsp::entities::context_rpc::TransferRpcBody;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::http::rpc::adapter::TransferRpcAdapter;
use crate::protocols::dsp::setup::context::DspContext;

/// What the TCK posts to `dataspacetck.dsp.connector.transfer.initiate.url`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TckTransferInitiateRequest {
    pub agreement_id: String,
    pub format: String,
    pub provider_id: String,
    pub connector_address: String,
}

#[derive(Clone)]
pub struct TransferRpcRouter {
    ctx: Arc<DspContext>,
}

impl TransferRpcRouter {
    pub fn new(ctx: Arc<DspContext>) -> Self {
        Self { ctx }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/rpc/setup-request", post(Self::handle_setup_request))
            .route("/rpc/setup-start", post(Self::handle_setup_start))
            .route("/rpc/setup-completion", post(Self::handle_setup_completion))
            .route("/rpc/setup-suspension", post(Self::handle_setup_suspension))
            .route(
                "/rpc/setup-termination",
                post(Self::handle_setup_termination),
            )
            .with_state(self)
    }

    /// The TCK's consumer trigger; mounted only in TCK mode, and run as the agent itself.
    pub fn tck_router(self) -> Router {
        Router::new()
            .route("/tck/transfers/requests", post(Self::handle_tck_initiate))
            .with_state(self)
    }

    /// Runs the outbound flow for `key`; the adapter reads the user and the JSON body.
    async fn dispatch(&self, key: TransferDSPMessageType, request: Request) -> Response {
        self.ctx
            .outbound
            .serve(&TransferRpcAdapter, key, request)
            .await
    }

    async fn handle_setup_request(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(TransferDSPMessageType::TransferRequestMessage, request)
            .await
    }

    async fn handle_setup_start(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(TransferDSPMessageType::TransferStartMessage, request)
            .await
    }

    async fn handle_setup_completion(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(TransferDSPMessageType::TransferCompletionMessage, request)
            .await
    }

    async fn handle_setup_suspension(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(TransferDSPMessageType::TransferSuspensionMessage, request)
            .await
    }

    async fn handle_setup_termination(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(TransferDSPMessageType::TransferTerminationMessage, request)
            .await
    }

    /// The TCK's consumer trigger, translated to a request towards the TCK as provider. The
    /// callback is where the TCK reached us, which our configured host may not be (Docker).
    async fn handle_tck_initiate(
        State(s): State<Self>,
        headers: HeaderMap,
        body: Result<Json<TckTransferInitiateRequest>, JsonRejection>,
    ) -> Response {
        let tck = match body {
            Ok(Json(tck)) => tck,
            Err(rejection) => {
                return (StatusCode::BAD_REQUEST, rejection.body_text()).into_response()
            }
        };
        let body = TransferRpcBody {
            associated_agent_peer: Some(tck.provider_id),
            agreement_id: Some(tck.agreement_id),
            format: Some(tck.format),
            provider_address: Some(tck.connector_address),
            callback_address: headers
                .get(HOST)
                .and_then(|host| host.to_str().ok())
                .map(|host| format!("http://{host}{}", paths::DSP_CURRENT)),
            ..TransferRpcBody::default()
        }
        .to_json_ld(&TransferDSPMessageType::TransferRequestMessage);
        let mut request = Request::new(Body::from(body.to_string()));
        request
            .extensions_mut()
            .insert(DspCaller::User(UserInfo::system()));
        s.dispatch(TransferDSPMessageType::TransferRequestMessage, request)
            .await
    }
}
