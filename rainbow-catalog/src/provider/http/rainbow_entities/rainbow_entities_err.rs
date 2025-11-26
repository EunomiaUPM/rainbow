/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use crate::provider::core::rainbow_entities::rainbow_catalog_err::{CatalogError, CatalogErrorOut};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

impl IntoResponse for CatalogError {
    fn into_response(self) -> Response {
        match self {
            e @ CatalogError::NotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(CatalogErrorOut::new(
                    "404".to_string(),
                    "NOT_FOUND".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CatalogError::DbErr(..) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CatalogErrorOut::new(
                    "500".to_string(),
                    "INTERNAL_SERVER_ERROR".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CatalogError::ConversionError(..) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CatalogErrorOut::new(
                    "500".to_string(),
                    "INTERNAL_SERVER_ERROR".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CatalogError::DataServiceNotFoundForDistribution { .. } => (
                StatusCode::BAD_REQUEST,
                Json(CatalogErrorOut::new(
                    "400".to_string(),
                    "BAD_REQUEST".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CatalogError::UrnUuidSchema(..) => (
                StatusCode::BAD_REQUEST,
                Json(CatalogErrorOut::new(
                    "400".to_string(),
                    "BAD_REQUEST".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CatalogError::DctFormatSchema(..) => (
                StatusCode::BAD_REQUEST,
                Json(CatalogErrorOut::new(
                    "400".to_string(),
                    "BAD_REQUEST".to_string(),
                    e.to_string(),
                )),
            ),
            e @ CatalogError::JsonRejection(..) => (
                StatusCode::BAD_REQUEST,
                Json(CatalogErrorOut::new(
                    "400".to_string(),
                    "BAD_REQUEST".to_string(),
                    e.to_string(),
                )),
            ),
        }
        .into_response()
    }
}
