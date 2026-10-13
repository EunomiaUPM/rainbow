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
//! `RpcTransferResult` or a `TransferError` out.

use axum::response::{IntoResponse, Response};
use axum::Json;
use common::flow::{FlowAdapterTrait, FlowError};
use http::StatusCode;
use serde_json::Value;

use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::transfer_error::TransferErrorDto;
use crate::protocols::dsp::entities::context_rpc::{
    TransferRpcOutcome, TransferRpcRaw, TransferRpcRdf, TransferRpcTyped,
};
use crate::protocols::dsp::services::rpc_flow::service::TransferRpcFlow;
use crate::protocols::dsp::services::rpc_flow::views::TransferRpcResult;

/// Pids the error names, as the caller sent them.
#[derive(Debug, Default)]
pub struct TransferRpcErrorPids {
    consumer_pid: String,
    provider_pid: String,
}

/// Extracts the RPC body for the route's message and answers with the result.
pub struct TransferRpcAdapter;

#[async_trait::async_trait]
impl FlowAdapterTrait<TransferRpcFlow> for TransferRpcAdapter {
    type Raw = Value;
    type Response = Response;
    type ErrorContext = TransferRpcErrorPids;

    /// Raw body, expanded through the eunomia context, then read into its fields.
    async fn extract(
        &self,
        key: &TransferDSPMessageType,
        raw: Value,
    ) -> Result<TransferRpcTyped, FlowError> {
        let raw = TransferRpcRaw::new(key, raw);
        let rdf = TransferRpcRdf::from_raw(raw).await?;
        Ok(TransferRpcTyped::from_rdf(rdf)?)
    }

    fn error_context(&self, input: &TransferRpcTyped) -> TransferRpcErrorPids {
        TransferRpcErrorPids {
            consumer_pid: input.body.consumer_pid.clone().unwrap_or_default(),
            provider_pid: input.body.provider_pid.clone().unwrap_or_default(),
        }
    }

    /// 201 when a request opened a process, 200 for a state change.
    fn respond(&self, key: &TransferDSPMessageType, output: TransferRpcOutcome) -> Response {
        let status = match key {
            TransferDSPMessageType::TransferRequestMessage => StatusCode::CREATED,
            _ => StatusCode::OK,
        };
        (status, Json(TransferRpcResult::assemble(output))).into_response()
    }

    fn reject(
        &self,
        _key: &TransferDSPMessageType,
        pids: TransferRpcErrorPids,
        error: FlowError,
    ) -> Response {
        let status =
            StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = TransferErrorDto::new(
            pids.consumer_pid,
            pids.provider_pid,
            status.as_u16().to_string(),
            error.reasons(),
        );
        (status, Json(body)).into_response()
    }
}
