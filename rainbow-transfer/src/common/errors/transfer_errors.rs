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
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rainbow_common::errors::{ErrorInfo, ErrorLog};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum TransferErrors {
    #[error("Consumer already registered")]
    ConsumerAlreadyRegistered {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: u16,
        cause: Option<String>,
    },
    #[error("Protocol Error. This message type is not allowed")]
    ProtocolError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: u16,
        cause: Option<String>,
    },
}

impl IntoResponse for &TransferErrors {
    fn into_response(self) -> Response {
        let info = match self {
            TransferErrors::ConsumerAlreadyRegistered { info, .. } => info,
            TransferErrors::ProtocolError { info, .. } => info,
        };
        (info.status_code, Json(info)).into_response()
    }
}

impl ErrorLog for TransferErrors {
    fn log(&self) -> String {
        match self {
            TransferErrors::ConsumerAlreadyRegistered { info, cause, http_code } => {
                let http_code = format!("Http Code: {}", http_code);
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                format!(
                    "\n{}\n Error Code: {}\n Message: {}\n {}\n {}\n{}",
                    self, http_code, info.error_code, info.message, details, cause
                )
            }
            TransferErrors::ProtocolError { info, cause, http_code } => {
                let http_code = format!("Http Code: {}", http_code);
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );
                format!(
                    "\n{}\n Error Code: {}\n Message: {}\n {}\n {}\n{}",
                    self, http_code, info.error_code, info.message, details, cause
                )
            }
        }
    }
}

impl TransferErrors {
    pub fn consumer_already_registered_new(id: Option<String>, cause: Option<String>) -> TransferErrors {
        TransferErrors::ConsumerAlreadyRegistered {
            info: ErrorInfo {
                message: "Consumer has been already declared".to_string(),
                error_code: 2100,
                status_code: StatusCode::BAD_REQUEST,
                details: format!(
                    "Consumer {} has been declared",
                    id.unwrap_or("".to_string())
                )
                .into(),
            },
            http_code: 400,
            cause,
        }
    }
    pub fn protocol_new(cause: Option<String>) -> TransferErrors {
        TransferErrors::ProtocolError {
            info: ErrorInfo {
                message: "Protocol Error. This message type is not allowed".to_string(),
                error_code: 2100,
                status_code: StatusCode::BAD_REQUEST,
                details: None,
            },
            http_code: 400,
            cause,
        }
    }
}
