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
use tracing::error;

#[derive(Error, Debug, Serialize)]
pub enum AuthErrors {
    #[error("Wallet Error")]
    WalletError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: u16,
        url: String,
        method: String,
        cause: Option<String>,
    },
    #[error("Security Error")]
    SecurityError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: Option<String>,
    },
}

impl IntoResponse for &AuthErrors {
    fn into_response(self) -> Response {
        let info = match self {
            AuthErrors::WalletError { info, .. } | AuthErrors::SecurityError { info, .. } => info,
        };

        (info.status_code, Json(info)).into_response()
    }
}

impl ErrorLog for AuthErrors {
    fn log(&self) -> String {
        match self {
            AuthErrors::WalletError { info, http_code, url, method, cause } => {
                let http_code = format!("Http Code: {}", http_code);
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));

                format!(
                    "\n{}\n Method: {}\n Url: {}\n {}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, method, url, http_code, info.error_code, info.message, details, cause
                )
            }
            AuthErrors::SecurityError { info, cause } => {
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                format!(
                    "\n{}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, info.error_code, info.message, details, cause
                )
            }
        }
    }
}

impl AuthErrors {
    pub fn wallet_new(url: String, method: String, http_code: u16, cause: Option<String>) -> AuthErrors {
        AuthErrors::WalletError {
            info: ErrorInfo {
                message: "Unexpected response from the Wallet".to_string(),
                error_code: 2100,
                status_code: StatusCode::BAD_GATEWAY,
                details: None,
            },
            http_code,
            url,
            method,
            cause,
        }
    }
    pub fn security_new(cause: Option<String>) -> AuthErrors {
        AuthErrors::SecurityError {
            info: ErrorInfo {
                message: "Invalid petition".to_string(),
                error_code: 4400,
                status_code: StatusCode::UNPROCESSABLE_ENTITY,
                details: None,
            },
            cause,
        }
    }
}
