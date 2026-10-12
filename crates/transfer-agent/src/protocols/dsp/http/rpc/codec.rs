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

//! JSON-LD over HTTP for the outbound flow: the eunomia RPC body in, the eunomia result or a
//! `TransferError` out.

use axum::response::{IntoResponse, Response};
use axum::Json;
use common::flow::{FlowCodecTrait, FlowError};
use http::StatusCode;
use serde_json::Value;

use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::transfer_error::TransferErrorDto;
use crate::protocols::dsp::entities::context_rpc::TransferRpcCommand;
use crate::protocols::dsp::services::rpc_flow::service::TransferRpcFlow;
use crate::protocols::dsp::services::rpc_flow::views::TransferRpcReply;

/// Pids the error names, as the caller sent them.
#[derive(Debug, Default)]
pub struct TransferRpcErrorPids {
    consumer_pid: String,
    provider_pid: String,
}

/// Reads the RPC body for the route's message and writes the answer.
pub struct TransferRpcCodec;

#[async_trait::async_trait]
impl FlowCodecTrait<TransferRpcFlow> for TransferRpcCodec {
    type Raw = Value;
    type Response = Response;
    type ErrorContext = TransferRpcErrorPids;

    async fn decode(
        &self,
        key: &TransferDSPMessageType,
        raw: Value,
    ) -> Result<TransferRpcCommand, FlowError> {
        TransferRpcCommand::read(key, raw)
            .await
            .map_err(FlowError::from)
    }

    fn error_context(&self, input: &TransferRpcCommand) -> TransferRpcErrorPids {
        TransferRpcErrorPids {
            consumer_pid: input.body.consumer_pid.clone().unwrap_or_default(),
            provider_pid: input.body.provider_pid.clone().unwrap_or_default(),
        }
    }

    /// 201 when a request opened a process, 200 for a state change.
    fn encode(&self, key: &TransferDSPMessageType, output: TransferRpcReply) -> Response {
        let status = match key {
            TransferDSPMessageType::TransferRequestMessage => StatusCode::CREATED,
            _ => StatusCode::OK,
        };
        (status, Json(output)).into_response()
    }

    fn encode_error(
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
