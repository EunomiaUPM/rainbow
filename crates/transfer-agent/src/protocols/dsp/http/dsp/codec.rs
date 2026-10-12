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

//! DSP over HTTP for the transfer flow: request bytes in, `TransferProcess` or `TransferError` out.

use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use axum::Json;
use common::flow::{FlowCodecTrait, FlowError};
use http::StatusCode;

use crate::entities::protocol::ProtocolId;
use crate::protocols::dsp::entities::context_dsp::TransferDspInput;
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::transfer_error::TransferErrorDto;
use crate::protocols::dsp::services::dsp_flow::service::TransferDspFlow;
use crate::protocols::dsp::services::dsp_flow::views::TransferDspReply;

/// Pids a `TransferError` names, taken from the input before the flow runs.
#[derive(Debug, Default)]
pub struct TransferErrorPids {
    consumer_pid: String,
    provider_pid: String,
}

/// Reads a DSP 2025-1 transfer message through RDF and writes the answer.
pub struct TransferDspCodec;

#[async_trait::async_trait]
impl FlowCodecTrait<TransferDspFlow> for TransferDspCodec {
    type Raw = Request;
    type Response = Response;
    type ErrorContext = TransferErrorPids;

    async fn decode(
        &self,
        key: &TransferDSPMessageType,
        raw: Request,
    ) -> Result<TransferDspInput, FlowError> {
        TransferDspInput::read(&ProtocolId::Dsp2025_1, key, raw)
            .await
            .map_err(FlowError::from)
    }

    fn error_context(&self, input: &TransferDspInput) -> TransferErrorPids {
        TransferErrorPids {
            consumer_pid: input.body.fields.consumer_pid.clone().unwrap_or_default(),
            provider_pid: input.body.fields.provider_pid.clone().unwrap_or_default(),
        }
    }

    /// DSP 10.2: 201 with the `TransferProcess` for a request, 200 for a state change.
    fn encode(&self, _key: &TransferDSPMessageType, output: TransferDspReply) -> Response {
        if output.created {
            (StatusCode::CREATED, Json(output.ack)).into_response()
        } else {
            StatusCode::OK.into_response()
        }
    }

    fn encode_error(
        &self,
        _key: &TransferDSPMessageType,
        pids: TransferErrorPids,
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
