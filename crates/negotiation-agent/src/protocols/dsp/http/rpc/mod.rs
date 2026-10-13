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

//! Eunomia RPC routes a local user calls to make the agent send negotiation messages, and the
//! endpoint the DSP TCK uses to make a consumer open a negotiation (TCK mode only).

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
use serde_json::{json, Value};

use crate::protocols::dsp::entities::context_rpc::NegotiationRpcBody;
use crate::protocols::dsp::entities::message_types::NegotiationRpcMessage;
use crate::protocols::dsp::http::rpc::adapter::NegotiationRpcAdapter;
use crate::protocols::dsp::setup::context::DspContext;

/// What the TCK posts to `dataspacetck.dsp.connector.negotiation.initiate.url`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TckNegotiationInitiateRequest {
    pub dataset_id: String,
    pub offer_id: String,
    pub provider_id: String,
    pub connector_address: String,
}

#[derive(Clone)]
pub struct NegotiationRpcRouter {
    ctx: Arc<DspContext>,
}

impl NegotiationRpcRouter {
    pub fn new(ctx: Arc<DspContext>) -> Self {
        Self { ctx }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/rpc/setup-request-init", post(Self::handle_request_init))
            .route("/rpc/setup-request", post(Self::handle_request))
            .route("/rpc/setup-offer-init", post(Self::handle_offer_init))
            .route("/rpc/setup-offer", post(Self::handle_offer))
            .route("/rpc/setup-acceptance", post(Self::handle_acceptance))
            .route("/rpc/setup-agreement", post(Self::handle_agreement))
            .route("/rpc/setup-verification", post(Self::handle_verification))
            .route("/rpc/setup-finalization", post(Self::handle_finalization))
            .route("/rpc/setup-termination", post(Self::handle_termination))
            .with_state(self)
    }

    /// The TCK's consumer trigger; mounted only in TCK mode, and run as the agent itself.
    pub fn tck_router(self) -> Router {
        Router::new()
            .route(
                "/tck/negotiations/requests",
                post(Self::handle_tck_initiate),
            )
            .with_state(self)
    }

    /// Runs the outbound flow for `key`; the adapter reads the user and the JSON body.
    pub async fn dispatch(&self, key: NegotiationRpcMessage, request: Request) -> Response {
        self.ctx
            .outbound
            .serve(&NegotiationRpcAdapter, key, request)
            .await
    }

    async fn handle_request_init(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationRpcMessage::RequestInit, request)
            .await
    }

    async fn handle_request(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationRpcMessage::Request, request).await
    }

    async fn handle_offer_init(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationRpcMessage::OfferInit, request).await
    }

    async fn handle_offer(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationRpcMessage::Offer, request).await
    }

    async fn handle_acceptance(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationRpcMessage::Acceptance, request).await
    }

    async fn handle_agreement(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationRpcMessage::Agreement, request).await
    }

    async fn handle_verification(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationRpcMessage::Verification, request)
            .await
    }

    async fn handle_finalization(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationRpcMessage::Finalization, request)
            .await
    }

    async fn handle_termination(State(s): State<Self>, request: Request) -> Response {
        s.dispatch(NegotiationRpcMessage::Termination, request)
            .await
    }

    /// The TCK's consumer trigger, translated to a request towards the TCK as provider. The
    /// callback is where the TCK reached us, which our configured host may not be (Docker).
    async fn handle_tck_initiate(
        State(s): State<Self>,
        headers: HeaderMap,
        body: Result<Json<TckNegotiationInitiateRequest>, JsonRejection>,
    ) -> Response {
        let tck = match body {
            Ok(Json(tck)) => tck,
            Err(rejection) => {
                return (StatusCode::BAD_REQUEST, rejection.body_text()).into_response()
            }
        };
        let body = NegotiationRpcBody {
            associated_agent_peer: Some(tck.provider_id),
            provider_address: Some(tck.connector_address),
            callback_address: headers
                .get(HOST)
                .and_then(|host| host.to_str().ok())
                .map(|host| format!("http://{host}{}", paths::DSP_CURRENT)),
            offer: Some(Self::tck_offer(&tck.offer_id, &tck.dataset_id)),
            ..NegotiationRpcBody::default()
        }
        .to_json_ld(NegotiationRpcMessage::RequestInit);
        let mut request = Request::new(Body::from(body.to_string()));
        request
            .extensions_mut()
            .insert(DspCaller::User(UserInfo::system()));
        s.dispatch(NegotiationRpcMessage::RequestInit, request)
            .await
    }

    /// The offer as the TCK itself sends one: `use` on the dataset (DSP 2025-1 requires the
    /// type and rules, not just the id). The TCK makes the id up (`offer...`), so it is kept as is.
    pub fn tck_offer(offer_id: &str, dataset_id: &str) -> Value {
        json!({
            "@id": offer_id,
            "@type": "Offer",
            "target": dataset_id,
            "permission": [{ "action": "use" }],
        })
    }
}
