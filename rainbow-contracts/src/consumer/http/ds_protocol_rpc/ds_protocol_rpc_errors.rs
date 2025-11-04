/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_errors::DSRPCContractNegotiationConsumerErrors;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rainbow_common::protocol::contract::contract_error::ContractErrorMessage;

impl IntoResponse for DSRPCContractNegotiationConsumerErrors {
    fn into_response(self) -> Response {
        match self {
            ref e @ DSRPCContractNegotiationConsumerErrors::ProviderNotReachable {
                ref provider_pid,
                ref consumer_pid,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                    consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                    code: Some("PROVIDER_NOT_REACHABLE".to_string()),
                    reason: Some(vec![e.to_string()]),
                    ..Default::default()
                }),
            ),
            ref e @ DSRPCContractNegotiationConsumerErrors::ProviderInternalError {
                ref provider_pid,
                ref consumer_pid,
                ref error,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                    consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                    code: Some("PROVIDER_INTERNAL_ERROR".to_string()),
                    reason: Some(vec![
                        e.to_string(),
                        error.clone().unwrap_or_default().to_string(),
                    ]),
                    ..Default::default()
                }),
            ),
            ref e @ DSRPCContractNegotiationConsumerErrors::ProviderResponseNotSerializable {
                ref provider_pid,
                ref consumer_pid,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                    consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                    code: Some("PROVIDER_RESPONSE_NOT_SERIALIZABLE".to_string()),
                    reason: Some(vec![e.to_string()]),
                    ..Default::default()
                }),
            ),
            ref e @ DSRPCContractNegotiationConsumerErrors::DSProtocolContractNegotiationError(..) => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: None,
                    consumer_pid: None,
                    code: Some("DS_PROTOCOL_ERROR".to_string()),
                    reason: Some(vec![e.to_string()]),
                    ..Default::default()
                }),
            ),
            ref e @ DSRPCContractNegotiationConsumerErrors::ConsumerAndProviderCorrelationError {
                ref provider_pid,
                ref consumer_pid,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: Option::from(provider_pid.to_owned().to_string()),
                    consumer_pid: Option::from(consumer_pid.to_owned().to_string()),
                    code: Some("CONSUMER_PROVIDER_CORRELATION_ERROR".to_string()),
                    reason: Some(vec![e.to_string()]),
                    ..Default::default()
                }),
            ),

            e @ DSRPCContractNegotiationConsumerErrors::OdrlValidationError => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: None,
                    consumer_pid: None,
                    code: Some("ODRL_VALIDATION_ERROR".to_string()),
                    reason: Some(vec![e.to_string()]),
                    ..Default::default()
                }),
            ),
        }
        .into_response()
    }
}
