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
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rainbow_common::protocol::transfer::transfer_error::TransferError;

impl IntoResponse for DSProtocolTransferConsumerErrors {
    fn into_response(self) -> Response {
        match self {
            DSProtocolTransferConsumerErrors::DbErr(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TransferError {
                    provider_pid: None,
                    consumer_pid: None,
                    code: "DATABASE_ERROR".to_string(),
                    reason: vec![
                        e.to_string()
                    ],
                    ..Default::default()
                })
            ),
            ref e @ DSProtocolTransferConsumerErrors::TransferProcessNotFound { ref provider_pid, .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TransferError {
                    provider_pid: provider_pid.clone().map(|pid| pid.to_string()),
                    consumer_pid: None,
                    code: "TRANSFER_PROCESS_NOT_FOUND".to_string(),
                    reason: vec![
                        e.to_string()
                    ],
                    ..Default::default()
                })
            ),
            ref e @ DSProtocolTransferConsumerErrors::DataAddressCannotBeNullOnPushError { ref consumer_pid, .. } => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    provider_pid: None,
                    consumer_pid: consumer_pid.clone().map(|cid| cid.to_string()),
                    code: "PROTOCOL_ERROR".to_string(),
                    reason: vec![
                        e.to_string()
                    ],
                    ..Default::default()
                })
            ),
            ref e @ DSProtocolTransferConsumerErrors::UriAndBodyIdentifiersDoNotCoincide => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    provider_pid: None,
                    consumer_pid: None,
                    code: "UUID_NOT_COINCIDE".to_string(),
                    reason: vec![
                        e.to_string()
                    ],
                    ..Default::default()
                })
            ),
            ref e @ DSProtocolTransferConsumerErrors::JsonRejection(ref _cause) => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    provider_pid: None,
                    consumer_pid: None,
                    code: "JSON_REJECTION".to_string(),
                    reason: vec![
                        e.to_string()
                    ],
                    ..Default::default()
                })
            ),
        }.into_response()
    }
}