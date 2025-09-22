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

use crate::provider::core::ds_protocol::ds_protocol_err::DSProtocolTransferProviderErrors;
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_err::DSRPCTransferProviderErrors;
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::DSRPCTransferProviderErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rainbow_common::protocol::transfer::transfer_error::TransferError;

impl IntoResponse for DSRPCTransferProviderErrors {
    fn into_response(self) -> Response {
        match self {
            ref e @ DSRPCTransferProviderErrors::ConsumerNotReachable { ref provider_pid, ref consumer_pid } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(DSRPCTransferProviderErrorResponse {
                    provider_pid: provider_pid.clone(),
                    consumer_pid: consumer_pid.clone(),
                    error: TransferError {
                        provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                        consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                        code: "CONSUMER_NOT_REACHABLE".to_string(),
                        reason: vec![e.to_string()],
                        ..Default::default()
                    },
                }),
            ),
            ref e @ DSRPCTransferProviderErrors::ConsumerInternalError { ref provider_pid, ref consumer_pid, ref error } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(DSRPCTransferProviderErrorResponse {
                    provider_pid: provider_pid.clone(),
                    consumer_pid: consumer_pid.clone(),
                    error: TransferError {
                        provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                        consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                        code: "CONSUMER_INTERNAL_ERROR".to_string(),
                        reason: vec![
                            e.to_string(),
                            error.clone().unwrap_or_default().to_string()
                        ],
                        ..Default::default()
                    },
                }),
            ),
            ref e @ DSRPCTransferProviderErrors::ConsumerResponseNotSerializable {
                ref provider_pid,
                ref consumer_pid,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(DSRPCTransferProviderErrorResponse {
                    provider_pid: provider_pid.clone(),
                    consumer_pid: consumer_pid.clone(),
                    error: TransferError {
                        provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                        consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                        code: "CONSUMER_INTERNAL_ERROR".to_string(),
                        reason: vec![e.to_string()],
                        ..Default::default()
                    },
                }),
            ),
            ref e @ DSRPCTransferProviderErrors::DSProtocolTransferProviderError(ref e_) => (
                StatusCode::BAD_REQUEST,
                Json(match e_ {
                    DSProtocolTransferProviderErrors::DbErr(_e__) => DSRPCTransferProviderErrorResponse {
                        provider_pid: None,
                        consumer_pid: None,
                        error: TransferError {
                            provider_pid: None,
                            consumer_pid: None,
                            code: "DATABASE_ERROR".to_string(),
                            reason: vec![e.to_string(), e_.to_string()],
                            ..Default::default()
                        },
                    },
                    ref _e__ @ DSProtocolTransferProviderErrors::TransferProcessNotFound {
                        ref provider_pid,
                        ref consumer_pid,
                    } => DSRPCTransferProviderErrorResponse {
                        provider_pid: provider_pid.clone(),
                        consumer_pid: consumer_pid.clone(),
                        error: TransferError {
                            provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                            consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                            code: "TRANSFER_PROCESS_NOT_FOUND".to_string(),
                            reason: vec![e.to_string(), e_.to_string()],
                            ..Default::default()
                        },
                    },
                    ref _e__ @ DSProtocolTransferProviderErrors::JsonRejection(ref _cause) => {
                        DSRPCTransferProviderErrorResponse {
                            provider_pid: None,
                            consumer_pid: None,
                            error: TransferError {
                                provider_pid: None,
                                consumer_pid: None,
                                code: "JSON_REJECTION".to_string(),
                                reason: vec![e.to_string(), e_.to_string()],
                                ..Default::default()
                            },
                        }
                    }
                    ref _e__ => DSRPCTransferProviderErrorResponse {
                        provider_pid: None,
                        consumer_pid: None,
                        error: TransferError {
                            provider_pid: None,
                            consumer_pid: None,
                            code: "DATABASE_ERROR".to_string(),
                            reason: vec![e.to_string(), e_.to_string()],
                            ..Default::default()
                        },
                    },
                }),
            ),
        }
            .into_response()
    }
}
