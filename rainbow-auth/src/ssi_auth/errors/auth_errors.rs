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

use axum::response::{IntoResponse, Response};
use axum::Json;
use rainbow_common::errors::ErrorInfo;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug, Serialize)]
pub enum AuthErrors {
    #[error("Petition Error")]
    ProviderError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: Option<u16>,
        url: String,
        method: String,
        cause: Option<String>,
    },
    #[error("Petition Error")]
    ConsumerError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: Option<String>,
    },
}

impl IntoResponse for &AuthErrors {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            AuthErrors::ProviderError { info, .. } => (StatusCode::BAD_GATEWAY, info),
            AuthErrors::ConsumerError { info, .. } => (StatusCode::BAD_REQUEST, info),
        };

        (status, Json(body)).into_response()
    }
}

impl AuthErrors {
    pub fn log(&self) {
        match self {
            AuthErrors::ProviderError { info, http_code, url, method, cause } => {
                let http_code = format!("Http Code: {}", http_code.unwrap_or(0));
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );
                error!("\n{}\n Method: {}\n Url: {}\n Http Code: {}\n Error Code: {}\n Message: {}\n Details: {}\n Cause: {}", self, method, url,http_code,info.error_code, info.message, details, cause);
            }
            AuthErrors::ConsumerError { info, cause } => {
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
