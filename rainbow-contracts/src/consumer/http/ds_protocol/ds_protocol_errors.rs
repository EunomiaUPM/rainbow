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

use crate::consumer::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use rainbow_common::protocol::contract::contract_error::ContractErrorMessage;

impl IntoResponse for IdsaCNError {
    fn into_response(self) -> Response {
        match self {
            e @ IdsaCNError::DbErr(..) => ContractErrorMessage {
                code: Option::from("DB_ERROR".to_string()),
                reason: Option::from(vec![e.to_string()]),
                ..Default::default()
            },
            ref e @ IdsaCNError::ProcessNotFound { ref provider_pid, .. } => {
                ContractErrorMessage {
                    provider_pid: provider_pid.as_ref().map(|pid| pid.to_string()),
                    code: Option::from("PROCESS_NOT_FOUND_BY_PROVIDER_OR_CONSUMER".to_string()),
                    reason: Option::from(vec![e.to_string()]),
                    ..Default::default()
                }
            }
            ref
            e @ IdsaCNError::UUIDParseError { ref provider_pid, ref error, .. } => {
                ContractErrorMessage {
                    provider_pid: provider_pid.as_ref().map(|pid| pid.to_string()),
                    code: Option::from("URN_PARSE_ERROR".to_string()),
                    reason: Option::from(vec![error.to_string(), e.to_string()]),
                    ..Default::default()
                }
            }
            ref e @ IdsaCNError::NotCheckedError {
                ref provider_pid,
                ref error,
                ..
            } => ContractErrorMessage {
                provider_pid: provider_pid.as_ref().map(|pid| pid.to_string()),
                code: Option::from("NOT_CHECKED_ERROR".to_string()),
                reason: Option::from(vec![
                    error.to_string(),
                    e.to_string(),
                    "Please contact data space connector admin".to_string(),
                ]),
                ..Default::default()
            },
            ref e @ IdsaCNError::JsonRejection(ref json_rejection) => match json_rejection {
                JsonRejection::JsonDataError(e_) => ContractErrorMessage {
                    code: Option::from("JSON_DATA_ERROR".to_string()),
                    reason: Option::from(vec![e_.body_text(), e.to_string()]),
                    ..Default::default()
                },
                JsonRejection::JsonSyntaxError(e_) => ContractErrorMessage {
                    code: Option::from("JSON_SYNTAX_ERROR".to_string()),
                    reason: Option::from(vec![e_.body_text(), e.to_string()]),
                    ..Default::default()
                },
                JsonRejection::MissingJsonContentType(e_) => ContractErrorMessage {
                    code: Option::from("MISSING_JSON_CONTENT_TYPE".to_string()),
                    reason: Option::from(vec![e_.body_text(), e.to_string()]),
                    ..Default::default()
                },
                JsonRejection::BytesRejection(e_) => ContractErrorMessage {
                    code: Option::from("JSON_BYTES_ERROR".to_string()),
                    reason: Option::from(vec![e_.body_text(), e.to_string()]),
                    ..Default::default()
                },
                e__ => ContractErrorMessage {
                    code: Option::from("NOT_CHECKED_ERROR".to_string()),
                    reason: Option::from(vec![e__.body_text(), e.to_string()]),
                    ..Default::default()
                },
            },
            e @ IdsaCNError::ValidationError(..) => ContractErrorMessage {
                code: Option::from("JSON_DATA_ERROR".to_string()),
                reason: Option::from(vec![e.to_string()]),
                ..Default::default()
            },
            ref e @ IdsaCNError::NotAllowed {
                ref provider_pid,
                ref consumer_pid,
                ..
            } => ContractErrorMessage {
                provider_pid: provider_pid.as_ref().map(|pid| pid.to_string()),
                consumer_pid: consumer_pid.as_ref().map(|pid| pid.to_string()),
                code: Option::from("OPERATION_NOT_ALLOWED".to_string()),
                reason: Option::from(vec![e.to_string()]),
                ..Default::default()
            },
        }
            .into_response()
    }
}
