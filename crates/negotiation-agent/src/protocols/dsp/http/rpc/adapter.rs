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

//! The outbound flow's HTTP adapter: the eunomia body through its stages in, the
//! `RpcNegotiationResult` or a `ContractNegotiationError` out.

use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use axum::Json;
use common::dsp_common::DspCaller;
use common::flow::{FlowAdapterTrait, FlowError};
use http::StatusCode;

use crate::protocols::dsp::entities::context_rpc::{
    NegotiationRpcOutcome, NegotiationRpcRaw, NegotiationRpcRdf, NegotiationRpcTyped,
};
use crate::protocols::dsp::entities::message_types::NegotiationRpcMessage;
use crate::protocols::dsp::entities::negotiation_error::NegotiationErrorDto;
use crate::protocols::dsp::services::rpc_flow::service::NegotiationRpcFlow;
use crate::protocols::dsp::services::rpc_flow::views::NegotiationRpcResult;

/// Pids the error names, as the caller sent them.
#[derive(Debug, Default)]
pub struct NegotiationRpcErrorPids {
    consumer_pid: String,
    provider_pid: String,
}

/// Extracts the RPC body for the route's message and answers with the result.
pub struct NegotiationRpcAdapter;

#[async_trait::async_trait]
impl FlowAdapterTrait<NegotiationRpcFlow> for NegotiationRpcAdapter {
    type Raw = Request;
    type Response = Response;
    type ErrorContext = NegotiationRpcErrorPids;

    /// The user OAuth resolved, then the JSON body, expanded through the eunomia context and
    /// read into its fields.
    async fn extract(
        &self,
        key: &NegotiationRpcMessage,
        raw: Request,
    ) -> Result<(DspCaller, NegotiationRpcTyped), FlowError> {
        let caller = DspCaller::from_extensions(raw.extensions())?;
        caller.user()?;
        let raw = NegotiationRpcRaw::from_request(*key, raw).await?;
        let rdf = NegotiationRpcRdf::from_raw(raw).await?;
        Ok((caller, NegotiationRpcTyped::from_rdf(rdf)?))
    }

    fn error_context(&self, input: &NegotiationRpcTyped) -> NegotiationRpcErrorPids {
        NegotiationRpcErrorPids {
            consumer_pid: input.body.consumer_pid.clone().unwrap_or_default(),
            provider_pid: input.body.provider_pid.clone().unwrap_or_default(),
        }
    }

    /// 201 when the message opened a negotiation, 200 for a state change.
    fn respond(&self, key: &NegotiationRpcMessage, output: NegotiationRpcOutcome) -> Response {
        let status = if key.opens() {
            StatusCode::CREATED
        } else {
            StatusCode::OK
        };
        (status, Json(NegotiationRpcResult::assemble(output))).into_response()
    }

    fn reject(
        &self,
        _key: &NegotiationRpcMessage,
        pids: NegotiationRpcErrorPids,
        error: FlowError,
    ) -> Response {
        let status =
            StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = NegotiationErrorDto::new(
            pids.consumer_pid,
            pids.provider_pid,
            status.as_u16().to_string(),
            error.reasons(),
        );
        (status, Json(body)).into_response()
    }
}
