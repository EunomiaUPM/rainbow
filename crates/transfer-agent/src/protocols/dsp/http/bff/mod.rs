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

//! The business route the GUI calls to open a transfer the provider starts on its own: the
//! RPC request with `autoStart` set, through the same outbound flow.

use std::sync::Arc;

use axum::body::{Body, Bytes};
use axum::extract::{Request, State};
use axum::response::Response;
use axum::routing::post;
use axum::Router;
use common::routes::transfer as routes;
use serde_json::Value;

use crate::protocols::dsp::entities::context_rpc::TransferRpcRaw;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::http::rpc::TransferRpcRouter;
use crate::protocols::dsp::setup::context::DspContext;

#[derive(Clone)]
pub struct TransferBffRouter {
    rpc: TransferRpcRouter,
}

impl TransferBffRouter {
    pub fn new(ctx: Arc<DspContext>) -> Self {
        Self {
            rpc: TransferRpcRouter::new(ctx),
        }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route(routes::bff_rpc::SETUP_REQUEST, post(Self::handle_request))
            .with_state(self)
    }

    async fn handle_request(State(s): State<Self>, request: Request) -> Response {
        let (parts, body) = request.into_parts();
        let body = axum::body::to_bytes(body, TransferRpcRaw::MAX_BODY_BYTES)
            .await
            .unwrap_or_default();
        let request = Request::from_parts(parts, Body::from(Self::with_auto_start(body)));
        s.rpc
            .dispatch(TransferDSPMessageType::TransferRequestMessage, request)
            .await
    }

    /// The body with `autoStart` set; anything but a JSON object goes on as it came, for the
    /// RPC flow to reject.
    fn with_auto_start(bytes: Bytes) -> Bytes {
        match serde_json::from_slice::<Value>(&bytes) {
            Ok(Value::Object(mut body)) => {
                body.insert("autoStart".to_string(), Value::Bool(true));
                serde_json::to_vec(&body).map(Bytes::from).unwrap_or(bytes)
            }
            _ => bytes,
        }
    }
}
