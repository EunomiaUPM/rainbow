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

pub mod codec;

use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use common::flow::FlowRunner;
use common::oauth::UserInfo;
use http::StatusCode;
use serde::Deserialize;
use serde_json::Value;

use crate::protocols::dsp::entities::context_rpc::TransferRpcBody;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::http::rpc::codec::TransferRpcCodec;
use crate::protocols::dsp::services::rpc_flow::service::TransferRpcFlow;

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
    runner: Arc<FlowRunner<TransferRpcFlow>>,
}

impl TransferRpcRouter {
    pub fn new(runner: Arc<FlowRunner<TransferRpcFlow>>) -> Self {
        Self { runner }
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

    /// Runs the outbound flow for `key` on the JSON body; a body that is not JSON is a 400.
    async fn dispatch(
        &self,
        user: &UserInfo,
        key: TransferDSPMessageType,
        body: Result<Json<Value>, JsonRejection>,
    ) -> Response {
        match body {
            Ok(Json(body)) => self.runner.serve(&TransferRpcCodec, user, key, body).await,
            Err(rejection) => (StatusCode::BAD_REQUEST, rejection.body_text()).into_response(),
        }
    }

    async fn handle_setup_request(
        State(s): State<Self>,
        user: UserInfo,
        body: Result<Json<Value>, JsonRejection>,
    ) -> Response {
        s.dispatch(&user, TransferDSPMessageType::TransferRequestMessage, body)
            .await
    }

    async fn handle_setup_start(
        State(s): State<Self>,
        user: UserInfo,
        body: Result<Json<Value>, JsonRejection>,
    ) -> Response {
        s.dispatch(&user, TransferDSPMessageType::TransferStartMessage, body)
            .await
    }

    async fn handle_setup_completion(
        State(s): State<Self>,
        user: UserInfo,
        body: Result<Json<Value>, JsonRejection>,
    ) -> Response {
        s.dispatch(
            &user,
            TransferDSPMessageType::TransferCompletionMessage,
            body,
        )
        .await
    }

    async fn handle_setup_suspension(
        State(s): State<Self>,
        user: UserInfo,
        body: Result<Json<Value>, JsonRejection>,
    ) -> Response {
        s.dispatch(
            &user,
            TransferDSPMessageType::TransferSuspensionMessage,
            body,
        )
        .await
    }

    async fn handle_setup_termination(
        State(s): State<Self>,
        user: UserInfo,
        body: Result<Json<Value>, JsonRejection>,
    ) -> Response {
        s.dispatch(
            &user,
            TransferDSPMessageType::TransferTerminationMessage,
            body,
        )
        .await
    }

    /// The TCK's consumer trigger, translated to a request towards the TCK as provider.
    async fn handle_tck_initiate(
        State(s): State<Self>,
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
            ..TransferRpcBody::default()
        }
        .to_json_ld(&TransferDSPMessageType::TransferRequestMessage);
        let agent = UserInfo::system();
        s.dispatch(
            &agent,
            TransferDSPMessageType::TransferRequestMessage,
            Ok(Json(body)),
        )
        .await
    }
}
