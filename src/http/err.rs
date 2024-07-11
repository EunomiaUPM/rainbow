use std::collections::VecDeque;
use std::fmt::Debug;

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use jsonschema::output::{ErrorDescription, OutputUnit};
use serde::Serialize;
use thiserror::Error;

use crate::transfer::messages::{TRANSFER_CONTEXT, TransferError, TransferMessageTypes};

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("The request body contains invalid data")]
    ValidationError {
        errors: VecDeque<OutputUnit<ErrorDescription>>
    },
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match &self {
            err @ HttpError::ValidationError { errors, .. } => {
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
        }.into_response()
    }
}

impl Serialize for HttpError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}