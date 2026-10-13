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

//! The inbound flow's HTTP adapter: the request through its extraction stages in, the
//! `TransferProcess` ACK or a `TransferError` out.

use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use axum::Json;
use common::dsp_common::DspCaller;
use common::flow::{FlowAdapterTrait, FlowError};
use http::StatusCode;

use crate::entities::protocol::ProtocolId;
use crate::protocols::dsp::entities::context_dsp::{
    TransferDspOutcome, TransferDspParsed, TransferDspRaw, TransferDspRdf, TransferDspTyped,
};
use crate::protocols::dsp::entities::message_types::TransferDSPMessageType;
use crate::protocols::dsp::entities::transfer_error::TransferErrorDto;
use crate::protocols::dsp::services::dsp_flow::service::TransferDspFlow;

/// Pids a `TransferError` names, taken from the input before the flow runs.
#[derive(Debug, Default)]
pub struct TransferErrorPids {
    consumer_pid: String,
    provider_pid: String,
}

/// Extracts a DSP 2025-1 transfer message stage by stage and answers with its outcome.
pub struct TransferDspAdapter;

#[async_trait::async_trait]
impl FlowAdapterTrait<TransferDspFlow> for TransferDspAdapter {
    type Raw = Request;
    type Response = Response;
    type ErrorContext = TransferErrorPids;

    /// The peer `PeerAuth` resolved, then raw request, JSON, RDF expansion, and the typed
    /// fields with the idempotency key.
    async fn extract(
        &self,
        key: &TransferDSPMessageType,
        raw: Request,
    ) -> Result<(DspCaller, TransferDspTyped), FlowError> {
        let caller = DspCaller::from_extensions(raw.extensions())?;
        let raw = TransferDspRaw::from_request(caller.peer()?.clone(), raw).await?;
        let parsed = TransferDspParsed::from_raw(raw, &ProtocolId::Dsp2025_1, key)?;
        let rdf = TransferDspRdf::from_parsed(parsed).await?;
        Ok((caller, TransferDspTyped::from_rdf(rdf)?))
    }

    fn error_context(&self, input: &TransferDspTyped) -> TransferErrorPids {
        TransferErrorPids {
            consumer_pid: input.fields.consumer_pid.clone().unwrap_or_default(),
            provider_pid: input.fields.provider_pid.clone().unwrap_or_default(),
        }
    }

    /// DSP 10.2: 201 with the `TransferProcess` for a request, 200 for a state change.
    fn respond(&self, _key: &TransferDSPMessageType, output: TransferDspOutcome) -> Response {
        if output.created {
            (StatusCode::CREATED, Json(output.ack)).into_response()
        } else {
            StatusCode::OK.into_response()
        }
    }

    fn reject(
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
