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

use crate::errors::error_log_trait::ErrorLogTrait;
use crate::types::enums::errors::{BadFormat, MissingAction};
use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    #[error("Provider Error")]
    ProviderError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: Option<u16>,
        url: String,
        method: String,
        cause: String,
    },
    #[error("Consumer Error")]
    ConsumerError {
        #[serde(flatten)]
        info: ErrorInfo,
        http_code: Option<u16>,
        url: String,
        method: String,
        cause: String,
    },
    #[error("Missing Action Error")]
    MissingActionError {
        #[serde(flatten)]
        info: ErrorInfo,
        action: MissingAction,
        cause: String,
    },
    #[error("Missing Resource Error")]
    MissingResourceError {
        #[serde(flatten)]
        info: ErrorInfo,
        resource_id: String,
        cause: String,
    },
    #[error("Format Error")]
    FormatError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: String,
    },
    #[error("Unauthorized")]
    UnauthorizedError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: String,
    },
    #[error("Forbidden")]
    ForbiddenError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: String,
    },
    #[error("Database Error")]
    DatabaseError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: String,
    },
    #[error("Feature Not Implemented Error")]
    FeatureNotImplError {
        #[serde(flatten)]
        info: ErrorInfo,
        feature: String,
        cause: String,
    },
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
    #[error("File Read Error")]
    ReadError {
        #[serde(flatten)]
        info: ErrorInfo,
        path: String,
        cause: String,
    },
    #[error("File Write Error")]
    WriteError {
        #[serde(flatten)]
        info: ErrorInfo,
        path: String,
        cause: String,
    },
    #[error("Parse Error")]
    ParseError {
        #[serde(flatten)]
        info: ErrorInfo,
        cause: String,
    },
}

impl Errors {
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
    pub fn provider_new(url: String, method: String, http_code: Option<u16>, cause: &str) -> Errors {
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
            cause: cause.to_string(),
        }
    }
    pub fn consumer_new(url: String, method: String, http_code: Option<u16>, cause: &str) -> Errors {
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
            cause: cause.to_string(),
        }
    }
    pub fn missing_action_new(action: MissingAction, cause: &str) -> Errors {
        let error_code = match action {
            MissingAction::Token => 3110,
            MissingAction::Wallet => 3120,
            MissingAction::Key => 3130,
            MissingAction::Did => 3140,
            MissingAction::Onboarding => 3150,
            _ => 3100,
        };
        Errors::MissingActionError {
            info: ErrorInfo {
                message: format!(
                    "The action {} is required to proceed with this step",
                    action
                ),
                error_code,
                status_code: StatusCode::PRECONDITION_FAILED,
                details: None,
            },
            action,
            cause: cause.to_string(),
        }
    }
    pub fn missing_resource_new(resource_id: String, cause: String) -> Errors {
        Errors::MissingResourceError {
            info: ErrorInfo {
                message: "Required resource missing".to_string(),
                error_code: 3200,
                status_code: StatusCode::NOT_FOUND,
                details: None,
            },
            resource_id,
            cause,
        }
    }
    pub fn format_new(option: BadFormat, cause: &str) -> Errors {
        let (error_code, status_code) = match option {
            BadFormat::Sent => (3110, StatusCode::BAD_GATEWAY),
            BadFormat::Received => (3120, StatusCode::BAD_REQUEST),
            _ => (3100, StatusCode::BAD_REQUEST),
        };
        Errors::FormatError {
            info: ErrorInfo { message: "Invalid Format".to_string(), error_code, status_code, details: None },
            cause: cause.to_string(),
        }
    }
    pub fn unauthorized_new(cause: &str) -> Errors {
        Errors::UnauthorizedError {
            info: ErrorInfo {
                message: "Unauthorized".to_string(),
                error_code: 4200,
                status_code: StatusCode::UNAUTHORIZED,
                details: None,
            },
            cause: cause.to_string(),
        }
    }
    pub fn forbidden_new(cause: &str) -> Errors {
        Errors::ForbiddenError {
            info: ErrorInfo {
                message: "Forbidden".to_string(),
                error_code: 4300,
                status_code: StatusCode::FORBIDDEN,
                details: None,
            },
            cause: cause.to_string(),
        }
    }
    pub fn database_new(cause: String) -> Errors {
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
    pub fn not_impl_new(feature: String, cause: &str) -> Errors {
        Errors::FeatureNotImplError {
            info: ErrorInfo {
                message: "Feature not implemented yet".to_string(),
                error_code: 5200,
                status_code: StatusCode::NOT_IMPLEMENTED,
                details: None,
            },
            feature,
            cause: cause.to_string(),
        }
    }
    pub fn wallet_new(url: String, method: String, http_code: u16, cause: &str) -> Errors {
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
            cause: cause.to_string(),
        }
    }
    pub fn security_new(cause: &str) -> Errors {
        Errors::SecurityError {
            info: ErrorInfo {
                message: "Invalid petition".to_string(),
                error_code: 4400,
                status_code: StatusCode::UNPROCESSABLE_ENTITY,
                details: None,
            },
            cause: cause.to_string(),
        }
    }
    pub fn read_new(path: String, cause: String) -> Self {
        Self::ReadError {
            info: ErrorInfo {
                message: format!("Failed to read file {}", path),
                error_code: 6010,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                details: None,
            },
            path,
            cause,
        }
    }

    pub fn write_new(path: String, cause: &str) -> Self {
        Self::WriteError {
            info: ErrorInfo {
                message: format!("Failed to write file {}", path),
                error_code: 6020,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                details: None,
            },
            path,
            cause: cause.to_string(),
        }
    }

    pub fn parse_new(cause: &str) -> Self {
        Self::ParseError {
            info: ErrorInfo {
                message: "Failed to parse file".to_string(),
                error_code: 6030,
                status_code: StatusCode::BAD_REQUEST,
                details: None,
            },
            cause: cause.to_string(),
        }
    }
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
            | Errors::ReadError { info, .. }
            | Errors::ParseError { info, .. }
            | Errors::WriteError { info, .. }
            | Errors::SecurityError { info, .. }
            | Errors::WalletError { info, .. } => info,
        };

        (info.status_code, Json(info)).into_response()
    }
}

impl ErrorLogTrait for Errors {
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
            Errors::PetitionError { info, http_code, url, method, cause }
            | Errors::ProviderError { info, http_code, url, method, cause }
            | Errors::ConsumerError { info, http_code, url, method, cause } => {
                format_http_error(info, url, method, *http_code, cause)
            }
            Errors::WalletError { info, http_code, url, method, cause } => {
                format_http_error(info, url, method, Some(*http_code), cause)
            }
            Errors::MissingActionError { info, action, cause } => {
                format!("{}\nMissingAction: {}", format_info(info, cause), action)
            }
            Errors::FeatureNotImplError { info, feature, cause } => {
                format!("{}\nFeature: {}", format_info(info, cause), feature)
            }
            Errors::MissingResourceError { info, resource_id, cause } => {
                format!("{}\nResource Id: {}", format_info(info, cause), resource_id)
            }
            Errors::ReadError { info, path, cause } => {
                format!("{}\nPath: {}", format_info(info, cause), path)
            }
            Errors::WriteError { info, path, cause } => {
                format!("{}\nPath: {}", format_info(info, cause), path)
            }
            Errors::FormatError { info, cause }
            | Errors::UnauthorizedError { info, cause }
            | Errors::ForbiddenError { info, cause }
            | Errors::DatabaseError { info, cause }
            | Errors::ParseError { info, cause }
            | Errors::SecurityError { info, cause } => format_info(info, cause),
        }
    }
}
