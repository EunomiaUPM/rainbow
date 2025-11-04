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

use crate::protocol::context_field::ContextField;
use crate::protocol::transfer::transfer_error::TransferError;
use crate::protocol::transfer::{TransferMessageTypes, TransferState};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use jsonschema::output::{ErrorDescription, OutputUnit};
use serde::Serialize;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransferErrorType {
    #[error("The transfer message is not compatible with the transfer protocol")]
    ValidationError { errors: VecDeque<OutputUnit<ErrorDescription>> },
    #[error("ConsumerPid, ProviderPid, AgreementId must have a valid Uuid v4")]
    PidUuidError,
    #[error("Identifiers must have urn:uuid:<uuid> schema")]
    PidSchemeError,
    #[error("Agreement doesn't seem to be valid")]
    AgreementError,
    #[error("Provider needs to have a valid uuid")]
    ProviderIdUuidError,
    #[error("DataAddress field cannot be null or undefined if dct:format is PUSH type")]
    DataAddressCannotBeNullOnPushError,
    #[error("Unknown TransferState")]
    UnknownTransferState,
    #[error("Unknown Callback")]
    CallbackClientError,
    #[error("Protocol Error. Consumer has been already registered")]
    ConsumerAlreadyRegisteredError,
    #[error("Protocol Error. Transfer process has been already suspended.")]
    TransferProcessAlreadySuspendedError,
    #[error("Protocol Error. Transfer process not found")]
    TransferProcessNotFound,
    #[error("Protocol Error. {message_type} is not allowed here. Current state is {state}")]
    ProtocolError { state: TransferState, message_type: String },
    #[error("Protocol Error. This message type is not allowed")]
    MessageTypeNotAcceptedError,
    #[error("Protocol Error. @type field is required. Check the documentation")]
    NoTypeFieldError,
    #[error("Not checked error.")]
    NotCheckedError { inner_error: anyhow::Error },
    #[error("It seems the consumer is not reachable")]
    ConsumerNotReachableError,
    #[error("It seems the provider is not reachable")]
    ProviderNotReachableError,
    #[error("It seems the provider or consumer peer is not recognized")]
    ProviderAndConsumerNotMatchingError,
    #[error("There is a problem parsing the TransferMessage")]
    ProtocolBodyError { message: String },
}

pub enum TransferErrorCodes {
    TransferErrorCode,
}

impl IntoResponse for TransferErrorType {
    fn into_response(self) -> Response {
        match &self {
            e @ TransferErrorType::ValidationError { errors, .. } => {
                let mut errors_out: Vec<String> = vec![];
                for error in errors {
                    errors_out.push(error.error_description().to_string())
                }
                errors_out.push(e.to_string());

                let error_message = TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: errors_out,
                };

                (StatusCode::BAD_REQUEST, Json(error_message))
            }
            e @ TransferErrorType::PidSchemeError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::PidUuidError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::ProviderIdUuidError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::AgreementError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::DataAddressCannotBeNullOnPushError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::UnknownTransferState => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::CallbackClientError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::ConsumerAlreadyRegisteredError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::TransferProcessNotFound => (
                StatusCode::NOT_FOUND,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::ProtocolError { .. } => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::MessageTypeNotAcceptedError { .. } => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::TransferProcessAlreadySuspendedError { .. } => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::NoTypeFieldError { .. } => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::NotCheckedError { inner_error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec!["Internal server error".to_string(), e.to_string(), inner_error.to_string()],
                }),
            ),
            e @ TransferErrorType::ConsumerNotReachableError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::ProviderNotReachableError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::ProtocolBodyError { message } => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string(), message.clone()],
                }),
            ),
            e @ TransferErrorType::ProviderAndConsumerNotMatchingError => (
                StatusCode::BAD_GATEWAY,
                Json(TransferError {
                    context: ContextField::default(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: None,
                    consumer_pid: None,
                    code: TransferErrorCodes::TransferErrorCode.to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
        }
        .into_response()
    }
}

impl Serialize for TransferErrorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl Display for TransferErrorCodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let code = match self {
            TransferErrorCodes::TransferErrorCode => "TRANSFER_ERROR_CODE",
        };
        write!(f, "{}", code)
    }
}
