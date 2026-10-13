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
//! `ContractNegotiation` ACK or a `ContractNegotiationError` out.

use axum::extract::Request;
use axum::response::{IntoResponse, Response};
use axum::Json;
use common::dsp_common::DspCaller;
use common::flow::{FlowAdapterTrait, FlowError};
use http::StatusCode;

use crate::protocols::dsp::entities::context_dsp::{
    NegotiationDspOutcome, NegotiationDspParsed, NegotiationDspRaw, NegotiationDspRdf,
    NegotiationDspTyped,
};
use crate::protocols::dsp::entities::message_types::NegotiationMessageType;
use crate::protocols::dsp::entities::negotiation_error::NegotiationErrorDto;
use crate::protocols::dsp::services::dsp_flow::service::NegotiationDspFlow;

/// Pids a `ContractNegotiationError` names, taken from the input before the flow runs.
#[derive(Debug, Default)]
pub struct NegotiationErrorPids {
    consumer_pid: String,
    provider_pid: String,
}

/// Extracts a DSP 2025-1 negotiation message stage by stage and answers with its outcome.
pub struct NegotiationDspAdapter;

#[async_trait::async_trait]
impl FlowAdapterTrait<NegotiationDspFlow> for NegotiationDspAdapter {
    type Raw = Request;
    type Response = Response;
    type ErrorContext = NegotiationErrorPids;

    /// The peer `PeerAuth` resolved, then raw request, JSON, RDF expansion, and the typed
    /// fields with the idempotency key.
    async fn extract(
        &self,
        key: &NegotiationMessageType,
        raw: Request,
    ) -> Result<(DspCaller, NegotiationDspTyped), FlowError> {
        let caller = DspCaller::from_extensions(raw.extensions())?;
        let raw = NegotiationDspRaw::from_request(caller.peer()?.clone(), raw).await?;
        let parsed = NegotiationDspParsed::from_raw(raw, *key)?;
        let rdf = NegotiationDspRdf::from_parsed(parsed).await?;
        Ok((caller, NegotiationDspTyped::from_rdf(rdf)?))
    }

    fn error_context(&self, input: &NegotiationDspTyped) -> NegotiationErrorPids {
        NegotiationErrorPids {
            consumer_pid: input.fields.consumer_pid.clone().unwrap_or_default(),
            provider_pid: input.fields.provider_pid.clone().unwrap_or_default(),
        }
    }

    /// The `ContractNegotiation`: 201 when a message opened one, 200 for a state change, whose
    /// body DSP leaves open.
    fn respond(&self, _key: &NegotiationMessageType, output: NegotiationDspOutcome) -> Response {
        let status = if output.created {
            StatusCode::CREATED
        } else {
            StatusCode::OK
        };
        (status, Json(output.ack)).into_response()
    }

    fn reject(
        &self,
        _key: &NegotiationMessageType,
        pids: NegotiationErrorPids,
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
