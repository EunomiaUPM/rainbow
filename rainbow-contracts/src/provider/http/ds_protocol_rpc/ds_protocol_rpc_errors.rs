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

use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_errors::DSRPCContractNegotiationProviderErrors;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rainbow_common::protocol::contract::contract_error::ContractErrorMessage;

impl IntoResponse for DSRPCContractNegotiationProviderErrors {
    fn into_response(self) -> Response {
        match self {
            ref e @ DSRPCContractNegotiationProviderErrors::ConsumerNotReachable {
                ref provider_pid,
                ref consumer_pid,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                    consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                    code: Some("CONSUMER_NOT_REACHABLE".to_string()),
                    reason: Some(vec![e.to_string()]),
                    ..Default::default()
                }),
            ),
            ref e @ DSRPCContractNegotiationProviderErrors::ConsumerInternalError {
                ref provider_pid,
                ref consumer_pid,
                ref consumer_error,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                    consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                    code: Some("CONSUMER_INTERNAL_ERROR".to_string()),
                    reason: Some(vec![e.to_string(), consumer_error.clone().to_string()]),
                    ..Default::default()
                }),
            ),
            ref e @ DSRPCContractNegotiationProviderErrors::ConsumerResponseNotSerializable {
                ref provider_pid,
                ref consumer_pid,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                    consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                    code: Some("CONSUMER_RESPONSE_NOT_SERIALIZABLE".to_string()),
                    reason: Some(vec![e.to_string()]),
                    ..Default::default()
                }),
            ),
            ref e @ DSRPCContractNegotiationProviderErrors::DSProtocolContractNegotiationError(..) => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: None,
                    consumer_pid: None,
                    code: Some("DS_PROTOCOL_ERROR".to_string()),
                    reason: Some(vec![e.to_string()]),
                    ..Default::default()
                }),
            ),
            ref e @ DSRPCContractNegotiationProviderErrors::ConsumerAndProviderCorrelationError {
                ref provider_pid,
                ref consumer_pid,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ContractErrorMessage {
                    provider_pid: Option::from(provider_pid.to_owned().to_string()),
                    consumer_pid: Option::from(consumer_pid.to_owned().to_string()),
                    code: Some("PROVIDER_CONSUMER_CORRELATION_ERROR".to_string()),
                    reason: Some(vec![e.to_string()]),
                    ..Default::default()
                }),
            ),

            e @ DSRPCContractNegotiationProviderErrors::OdrlValidationError => (
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
