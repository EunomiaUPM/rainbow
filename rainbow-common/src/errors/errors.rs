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
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorInfo {
    pub message: String,
    pub error_code: u16,
    pub details: Option<String>,
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CommonErrors {
    #[error("Petition Error")]
    PetitionError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: Option<u16>,
        url: String,
        method: String,
        cause: String,
    },

    #[error("Wallet Error")]
    WalletError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: Option<u16>,
        cause: Option<String>,
    },
    #[error("Format Error")]
    FormatError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: Option<String>,
    },
    #[error("Database Error")]
    DatabaseError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: Option<String>,
    },
    #[error("Object with id: {id} missing in the database")]
    MissingError {
        #[serde(flatten)]
        info: ErrorInfo,
        id: String,
        cause: Option<String>,
    },
    #[error("Invalid")]
    InvalidError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: Option<String>,
    },
}

impl IntoResponse for &CommonErrors {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            CommonErrors::PetitionError { info, .. } => (StatusCode::BAD_GATEWAY, info),
            CommonErrors::WalletError { info, .. } => (StatusCode::CONFLICT, info),
            CommonErrors::FormatError { info, .. } => (StatusCode::BAD_REQUEST, info),
            CommonErrors::DatabaseError { info, .. } => (StatusCode::BAD_REQUEST, info),
            CommonErrors::MissingError { info, .. } => (StatusCode::BAD_REQUEST, info),
            CommonErrors::InvalidError { info, .. } => (StatusCode::BAD_REQUEST, info),
        };

        (status, Json(body)).into_response()
    }
}

impl CommonErrors {
    pub fn log(&self) {
        match self {
            CommonErrors::PetitionError { info, http_code, url, method, cause } => {
                let http_code = format!("Http Code: {}", http_code.unwrap_or(0));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                error!(
                    "\n{}\n Method: {}\n Url: {}\n {}\n Error Code: {}\n Message: {}\n {}\n Cause: {}",
                    self, method, url, http_code, info.error_code, info.message, details, cause
                );
            }
            CommonErrors::WalletError { info, http_code, cause } => {
                let http_code = format!("Http Code: {}", http_code.unwrap_or(0));
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                error!(
                    "\n{}\n {}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, http_code, info.error_code, info.message, details, cause
                );
            }
            CommonErrors::FormatError { info, cause } => {
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                error!(
                    "\n{}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, info.error_code, info.message, details, cause
                );
            }
            CommonErrors::DatabaseError { info, cause } => {
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                error!(
                    "\n{}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, info.error_code, info.message, details, cause
                );
            }
            CommonErrors::MissingError { info, id, cause } => {
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                error!(
                    "\n{}\n Id: {}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, id, info.error_code, info.message, details, cause
                );
            }
            CommonErrors::InvalidError { info, cause } => {
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                error!(
                    "\n{}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, info.error_code, info.message, details, cause
                );
            }
        }
    }
}
