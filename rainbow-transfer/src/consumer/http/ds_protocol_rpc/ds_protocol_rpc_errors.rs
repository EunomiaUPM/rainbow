/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::consumer::core::ds_protocol::ds_protocol_err::DSProtocolTransferConsumerErrors;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_err::DSRPCTransferConsumerErrors;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::DSRPCTransferConsumerErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rainbow_common::protocol::transfer::transfer_error::TransferError;

impl IntoResponse for DSRPCTransferConsumerErrors {
    fn into_response(self) -> Response {
        match self {
            ref e @ DSRPCTransferConsumerErrors::ProviderNotReachable { ref provider_pid, ref consumer_pid } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(DSRPCTransferConsumerErrorResponse {
                    provider_pid: provider_pid.clone(),
                    consumer_pid: consumer_pid.clone(),
                    error: TransferError {
                        provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                        consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                        code: "PROVIDER_NOT_REACHABLE".to_string(),
                        reason: vec![e.to_string()],
                        ..Default::default()
                    },
                }),
            ),
            ref e @ DSRPCTransferConsumerErrors::ProviderInternalError { ref provider_pid, ref consumer_pid, ref error } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(DSRPCTransferConsumerErrorResponse {
                    provider_pid: provider_pid.clone(),
                    consumer_pid: consumer_pid.clone(),
                    error: TransferError {
                        provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                        consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                        code: "PROVIDER_INTERNAL_ERROR".to_string(),
                        reason: vec![e.to_string(), error.clone().unwrap().to_string()],
                        ..Default::default()
                    },
                }),
            ),
            ref e @ DSRPCTransferConsumerErrors::ProviderResponseNotSerializable {
                ref provider_pid,
                ref consumer_pid,
            } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(DSRPCTransferConsumerErrorResponse {
                    provider_pid: provider_pid.clone(),
                    consumer_pid: consumer_pid.clone(),
                    error: TransferError {
                        provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                        consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                        code: "PROVIDER_INTERNAL_ERROR".to_string(),
                        reason: vec![e.to_string()],
                        ..Default::default()
                    },
                }),
            ),
            ref e @ DSRPCTransferConsumerErrors::DSProtocolTransferConsumerError(ref e_) => (
                StatusCode::BAD_REQUEST,
                Json(match e_ {
                    DSProtocolTransferConsumerErrors::DbErr(_e__) => DSRPCTransferConsumerErrorResponse {
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
                    ref _e__ @ DSProtocolTransferConsumerErrors::TransferProcessNotFound {
                        ref provider_pid,
                        ref consumer_pid,
                    } => DSRPCTransferConsumerErrorResponse {
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
                    ref _e__ @ DSProtocolTransferConsumerErrors::JsonRejection(ref _cause) => {
                        DSRPCTransferConsumerErrorResponse {
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
                    ref _e__ => DSRPCTransferConsumerErrorResponse {
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
