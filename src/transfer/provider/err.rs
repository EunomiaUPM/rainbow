use std::collections::VecDeque;
use std::fmt::Debug;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use jsonschema::output::{ErrorDescription, OutputUnit};
use serde::Serialize;
use thiserror::Error;

use crate::transfer::protocol::messages::{TransferError, TransferMessageTypes, TRANSFER_CONTEXT};

#[derive(Error, Debug)]
pub enum TransferErrorType {
    #[error("The request body contains invalid data")]
    ValidationError {
        errors: VecDeque<OutputUnit<ErrorDescription>>,
    },
    #[error("Consumer needs to have a valid uuid")]
    ConsumerIdUuidError,
    #[error("Agreement doesn't seem to be valid")]
    AgreementError,
    #[error("Provider needs to have a valid uuid")]
    ProviderIdUuidError,
    #[error("DataAddress field cannot be null or undefined if dct:format is PUSH type")]
    DataAddressCannotBeNullOnPushError,
    // errores de procolo de transporte
    #[error("Unknown TransferState")]
    UnknownTransferState,
    #[error("Unknown Callback")]
    CallbackClientError,
}

impl IntoResponse for TransferErrorType {
    fn into_response(self) -> Response {
        match &self {
            TransferErrorType::ValidationError { errors, .. } => {
                let mut errors_out: Vec<String> = vec![];
                for error in errors {
                    errors_out.push(error.error_description().to_string())
                }

                let error_message = TransferError {
                    context: TRANSFER_CONTEXT.to_string(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: "123".to_string(),
                    consumer_pid: "123".to_string(),
                    code: "400".to_string(),
                    reason: errors_out,
                };

                (StatusCode::BAD_REQUEST, Json(error_message))
            }
            e @ TransferErrorType::ConsumerIdUuidError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: TRANSFER_CONTEXT.to_string(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: "123".to_string(),
                    consumer_pid: "123".to_string(),
                    code: "400".to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::ProviderIdUuidError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: TRANSFER_CONTEXT.to_string(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: "123".to_string(),
                    consumer_pid: "123".to_string(),
                    code: "400".to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::AgreementError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: TRANSFER_CONTEXT.to_string(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: "123".to_string(),
                    consumer_pid: "123".to_string(),
                    code: "400".to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::DataAddressCannotBeNullOnPushError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: TRANSFER_CONTEXT.to_string(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: "123".to_string(),
                    consumer_pid: "123".to_string(),
                    code: "400".to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::UnknownTransferState => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: TRANSFER_CONTEXT.to_string(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: "123".to_string(),
                    consumer_pid: "123".to_string(),
                    code: "400".to_string(),
                    reason: vec![e.to_string()],
                }),
            ),
            e @ TransferErrorType::CallbackClientError => (
                StatusCode::BAD_REQUEST,
                Json(TransferError {
                    context: TRANSFER_CONTEXT.to_string(),
                    _type: TransferMessageTypes::TransferError.to_string(),
                    provider_pid: "123".to_string(),
                    consumer_pid: "123".to_string(),
                    code: "400".to_string(),
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
