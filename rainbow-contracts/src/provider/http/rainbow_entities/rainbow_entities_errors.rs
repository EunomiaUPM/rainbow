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

use crate::provider::core::rainbow_entities::rainbow_entities_errors::{CnErrorProvider, CnErrorProviderErrorOut};
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

impl IntoResponse for CnErrorProvider {
    fn into_response(self) -> Response {
        match self {
            e @ CnErrorProvider::NotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(CnErrorProviderErrorOut::new(
                    "404".to_string(),
                    "NOT_FOUND".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CnErrorProvider::ProviderNotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(CnErrorProviderErrorOut::new(
                    "404".to_string(),
                    "NOT_FOUND".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CnErrorProvider::ConsumerNotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(CnErrorProviderErrorOut::new(
                    "404".to_string(),
                    "NOT_FOUND".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CnErrorProvider::ProcessNotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(CnErrorProviderErrorOut::new(
                    "404".to_string(),
                    "NOT_FOUND".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CnErrorProvider::DbErr(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CnErrorProviderErrorOut::new(
                    "500".to_string(),
                    "INTERNAL_SERVER_ERROR".to_string(),
                    e.to_string(),
                )),
            ),
            CnErrorProvider::JsonRejection(e) => match e {
                JsonRejection::JsonDataError(e_) => (
                    StatusCode::BAD_REQUEST,
                    Json(CnErrorProviderErrorOut::new(
                        "400".to_string(),
                        "BAD_REQUEST".to_string(),
                        format!("JsonDataError: {}", e_.body_text()),
                    )),
                ),
                JsonRejection::JsonSyntaxError(e_) => (
                    StatusCode::BAD_REQUEST,
                    Json(CnErrorProviderErrorOut::new(
                        "400".to_string(),
                        "BAD_REQUEST".to_string(),
                        format!("JsonSyntaxError: {}", e_.body_text()),
                    )),
                ),
                JsonRejection::MissingJsonContentType(e_) => (
                    StatusCode::BAD_REQUEST,
                    Json(CnErrorProviderErrorOut::new(
                        "400".to_string(),
                        "BAD_REQUEST".to_string(),
                        format!("MissingJsonContentType: {}", e_.body_text()),
                    )),
                ),
                JsonRejection::BytesRejection(e_) => (
                    StatusCode::BAD_REQUEST,
                    Json(CnErrorProviderErrorOut::new(
                        "400".to_string(),
                        "BAD_REQUEST".to_string(),
                        format!("BytesRejection: {}", e_.body_text()),
                    )),
                ),
                e_ => (
                    StatusCode::BAD_REQUEST,
                    Json(CnErrorProviderErrorOut::new(
                        "400".to_string(),
                        "BAD_REQUEST".to_string(),
                        e_.to_string(),
                    )),
                ),
            },
            e @ CnErrorProvider::UrnUuidSchema(_) => (
                StatusCode::BAD_REQUEST,
                Json(CnErrorProviderErrorOut::new(
                    "400".to_string(),
                    "BAD_REQUEST".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CnErrorProvider::LastNotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(CnErrorProviderErrorOut::new(
                    "404".to_string(),
                    "NOT_FOUND".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CnErrorProvider::MessageNotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(CnErrorProviderErrorOut::new(
                    "404".to_string(),
                    "NOT_FOUND".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CnErrorProvider::ParticipantNotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(CnErrorProviderErrorOut::new(
                    "404".to_string(),
                    "NOT_FOUND".to_string(),
                    e.to_string(),
                )),
            ),
        }
            .into_response()
    }
}
