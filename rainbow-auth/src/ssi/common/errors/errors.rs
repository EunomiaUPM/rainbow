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
use rainbow_common::errors::{ErrorInfo, ErrorLog};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum AuthErrors {
    #[error("Wallet Error")]
    WalletError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: u16,
        url: String,
        method: String,
        cause: String,
    },
    #[error("Security Error")]
    SecurityError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: String,
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
        fn format_info(info: &ErrorInfo, cause: &str) -> String {
            let details = info.details.as_deref().unwrap_or("No details");
            format!(
                "Error Code: {}\nMessage: {}\nDetails: {}\nCause: {}",
                info.error_code, info.message, details, cause
            )
        }

        fn format_http_error(info: &ErrorInfo, url: &str, method: &str, http_code: Option<u16>, cause: &str) -> String {
            let base = format_info(info, cause);
            let code = http_code.unwrap_or(0);
            format!(
                "{}\nMethod: {}\nUrl: {}\nHttp Code: {}",
                base, method, url, code
            )
        }
        match self {
            AuthErrors::WalletError { info, http_code, url, method, cause } => {
                format_http_error(info, url, method, Some(*http_code), cause)
            }
            AuthErrors::SecurityError { info, cause } => format_info(info, cause),
        }
    }
}

impl AuthErrors {
    pub fn wallet_new(url: &str, method: &str, http_code: u16, cause: &str) -> AuthErrors {
        AuthErrors::WalletError {
            info: ErrorInfo {
                message: "Unexpected response from the Wallet".to_string(),
                error_code: 2100,
                status_code: StatusCode::BAD_GATEWAY,
                details: None,
                cause: "".to_string(),
            },
            http_code,
            url: url.to_string(),
            method: method.to_string(),
            cause: cause.to_string(),
        }
    }
    pub fn security_new(cause: &str) -> AuthErrors {
        AuthErrors::SecurityError {
            info: ErrorInfo {
                message: "Invalid petition".to_string(),
                error_code: 4400,
                status_code: StatusCode::UNPROCESSABLE_ENTITY,
                details: None,
                cause: "".to_string(),
            },
            cause: cause.to_string(),
        }
    }
}
