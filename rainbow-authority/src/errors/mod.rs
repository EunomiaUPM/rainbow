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
mod error_adapter;
pub mod helpers;
pub use error_adapter::CustomToResponse;

use crate::errors::helpers::{BadFormat, MissingAction};
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
    #[serde(skip)]
    pub status_code: StatusCode,
    pub details: Option<String>,
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum Errors {
    #[error("Petition Error")]
    PetitionError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: Option<u16>,
        url: String,
        method: String,
        cause: String,
    },
    #[error("Petition Error")]
    ProviderError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: Option<u16>,
        url: Option<String>,
        method: Option<String>,
        cause: Option<String>,
    },
    #[error("Petition Error")]
    ConsumerError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: Option<u16>,
        url: Option<String>,
        method: Option<String>,
        cause: Option<String>,
    },
    #[error("Missing Action Error")]
    MissingActionError {
        #[serde(flatten)]
        info: ErrorInfo,
        action: String,
        cause: Option<String>,
    },
    #[error("Missing Resource Error")]
    MissingResourceError {
        #[serde(flatten)]
        info: ErrorInfo,
        resource_id: String,
        cause: Option<String>,
    },
    #[error("Format Error")]
    FormatError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: Option<String>,
    },
    #[error("Unauthorized")]
    UnauthorizedError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: Option<String>,
    },
    #[error("Forbidden")]
    ForbiddenError {
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
    #[error("Feature Not Implemented Error")]
    FeatureNotImplError {
        #[serde(flatten)]
        info: ErrorInfo,
        feature: String,
        cause: Option<String>,
    },
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

impl IntoResponse for &Errors {
    fn into_response(self) -> Response {
        let info = match self {
            Errors::PetitionError { info, .. }
            | Errors::ProviderError { info, .. }
            | Errors::ConsumerError { info, .. }
            | Errors::MissingActionError { info, .. }
            | Errors::MissingResourceError { info, .. }
            | Errors::FormatError { info, .. }
            | Errors::UnauthorizedError { info, .. }
            | Errors::ForbiddenError { info, .. }
            | Errors::DatabaseError { info, .. }
            | Errors::FeatureNotImplError { info, .. }
            | Errors::SecurityError { info, .. }
            | Errors::WalletError { info, .. } => info,
        };

        (info.status_code, Json(info)).into_response()
    }
}

impl Errors {
    pub fn log(&self) {
        match self {
            Errors::PetitionError { info, http_code, url, method, cause } => {
                let http_code = format!("Http Code: {}", http_code.unwrap_or(0));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                error!(
                    "\n{}\n Method: {}\n Url: {}\n {}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, method, url, http_code, info.error_code, info.message, details, cause
                );
            }
            Errors::ProviderError { info, http_code, url, method, cause } => {
                let http_code = format!("Http Code: {}", http_code.unwrap_or(0));
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let url = format!("Url: {}", url.as_deref().unwrap_or("No url"));
                let method = format!("Method: {}", method.as_deref().unwrap_or("No method"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );
                error!(
                    "\n{}\n {}\n {}\n {}\n Error Code: {}\n Message: {}\n Details: {}\n {}",
                    self, method, url, http_code, info.error_code, info.message, details, cause
                );
            }
            Errors::ConsumerError { info, http_code, url, method, cause } => {
                let http_code = format!("Http Code: {}", http_code.unwrap_or(0));
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let url = format!("Url: {}", url.as_deref().unwrap_or("No url"));
                let method = format!("Method: {}", method.as_deref().unwrap_or("No method"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );
                error!(
                    "\n{}\n {}\n {}\n {}\n Error Code: {}\n Message: {}\n Details: {}\n {}",
                    self, method, url, http_code, info.error_code, info.message, details, cause
                );
            }
            Errors::MissingActionError { info, action, cause } => {
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );
                error!(
                    "\n{}\n Error Code: {}\n Message: {}\n Details: {}\n MissingAction: {}\n {}",
                    self, info.error_code, info.message, details, action, cause
                );
            }
            Errors::MissingResourceError { info, resource_id, cause } => {
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                error!(
                    "\n{}\n Id: {}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, resource_id, info.error_code, info.message, details, cause
                );
            }
            Errors::FormatError { info, cause } => {
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
            Errors::UnauthorizedError { info, cause } => {
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
            Errors::ForbiddenError { info, cause } => {
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
            Errors::DatabaseError { info, cause } => {
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

            Errors::FeatureNotImplError { info, feature, cause } => {
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );

                error!(
                    "\n{}\n Feature: {}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, feature, info.error_code, info.message, details, cause
                );
            }
            Errors::WalletError { info, http_code, url, method, cause } => {
                let http_code = format!("Http Code: {}", http_code);
                let details = format!(
                    "Details: {}",
                    info.details.as_deref().unwrap_or("No details")
                );
                let cause = format!("Cause: {}", cause.as_deref().unwrap_or("No Cause"));

                error!(
                    "\n{}\n Method: {}\n Url: {}\n {}\n Error Code: {}\n Message: {}\n {}\n {}",
                    self, method, url, http_code, info.error_code, info.message, details, cause
                );
            }
            Errors::SecurityError { info, cause } => {
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

    pub fn petition_new(url: String, method: String, http_code: Option<u16>, cause: String) -> Errors {
        Errors::PetitionError {
            info: ErrorInfo {
                message: "A petition went wrong".to_string(),
                error_code: 1000,
                status_code: StatusCode::BAD_GATEWAY,
                details: None,
            },
            http_code,
            url,
            method,
            cause,
        }
    }
    pub fn provider_new(
        url: Option<String>,
        method: Option<String>,
        http_code: Option<u16>,
        cause: Option<String>,
    ) -> Errors {
        Errors::ProviderError {
            info: ErrorInfo {
                message: "Unexpected response from the Provider".to_string(),
                error_code: 2200,
                status_code: StatusCode::BAD_GATEWAY,
                details: None,
            },
            http_code,
            url,
            method,
            cause,
        }
    }
    pub fn consumer_new(
        url: Option<String>,
        method: Option<String>,
        http_code: Option<u16>,
        cause: Option<String>,
    ) -> Errors {
        Errors::ConsumerError {
            info: ErrorInfo {
                message: "Unexpected response from the Consumer".to_string(),
                error_code: 2300,
                status_code: StatusCode::BAD_GATEWAY,
                details: None,
            },
            http_code,
            url,
            method,
            cause,
        }
    }
    pub fn missing_action_new(action: String, missing: MissingAction, cause: Option<String>) -> Errors {
        let error_code = match missing {
            MissingAction::Token => 3110,
            MissingAction::Wallet => 3120,
            MissingAction::Did => 3130,
            MissingAction::Onboarding => 3140,
            _ => 3100,
        };
        Errors::MissingActionError {
            info: ErrorInfo {
                message: "An action is required to proceed with this step".to_string(),
                error_code,
                status_code: StatusCode::PRECONDITION_FAILED,
                details: Some(action.clone()),
            },
            action,
            cause,
        }
    }
    pub fn missing_resource_new(resource_id: String, cause: Option<String>) -> Errors {
        Errors::MissingResourceError {
            info: ErrorInfo {
                message: "A key resource is messing in order to complete the required action ".to_string(),
                error_code: 3200,
                status_code: StatusCode::NOT_FOUND,
                details: None,
            },
            resource_id,
            cause,
        }
    }
    pub fn format_new(option: BadFormat, cause: Option<String>) -> Errors {
        let (error_code, status_code) = match option {
            BadFormat::Sent => (3110, StatusCode::BAD_GATEWAY),
            BadFormat::Received => (3120, StatusCode::BAD_REQUEST),
            _ => (3100, StatusCode::BAD_REQUEST),
        };
        Errors::FormatError {
            info: ErrorInfo { message: "Invalid Format".to_string(), error_code, status_code, details: cause.clone() },
            cause,
        }
    }
    pub fn unauthorized_new(cause: Option<String>) -> Errors {
        Errors::UnauthorizedError {
            info: ErrorInfo {
                message: "Unauthorized".to_string(),
                error_code: 4200,
                status_code: StatusCode::UNAUTHORIZED,
                details: None,
            },
            cause,
        }
    }
    pub fn forbidden_new(cause: Option<String>) -> Errors {
        Errors::ForbiddenError {
            info: ErrorInfo {
                message: "Forbidden".to_string(),
                error_code: 4300,
                status_code: StatusCode::FORBIDDEN,
                details: None,
            },
            cause,
        }
    }
    pub fn database_new(cause: Option<String>) -> Errors {
        Errors::DatabaseError {
            info: ErrorInfo {
                message: "Error related to the database".to_string(),
                error_code: 5100,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                details: None,
            },
            cause,
        }
    }
    pub fn not_impl_new(feature: String, cause: Option<String>) -> Errors {
        Errors::FeatureNotImplError {
            info: ErrorInfo {
                message: "Feature not implemented yet".to_string(),
                error_code: 5200,
                status_code: StatusCode::NOT_IMPLEMENTED,
                details: None,
            },
            feature,
            cause,
        }
    }
    pub fn wallet_new(url: String, method: String, http_code: u16, cause: Option<String>) -> Errors {
        Errors::WalletError {
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
    pub fn security_new(cause: Option<String>) -> Errors {
        Errors::SecurityError {
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
