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

use crate::core::ds_protocol::ds_protocol_errors::DSProtocolCatalogErrors;
use crate::core::rainbow_entities::rainbow_catalog_err::CatalogErrorOut;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

impl IntoResponse for DSProtocolCatalogErrors {
    fn into_response(self) -> Response {
        match self {
            e @ DSProtocolCatalogErrors::NotFound { .. } => (
                StatusCode::NOT_FOUND,
                Json(CatalogErrorOut::new(
                    "404".to_string(),
                    "NOT_FOUND".to_string(),
                    e.to_string(),
                )),
            ),
            e @ DSProtocolCatalogErrors::DbErr(..) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CatalogErrorOut::new(
                    "500".to_string(),
                    "DB_ERROR".to_string(),
                    e.to_string(),
                )),
            ),
            e @ DSProtocolCatalogErrors::JsonRejection(..) => (
                StatusCode::BAD_REQUEST,
                Json(CatalogErrorOut::new(
                    "400".to_string(),
                    "JSON_REJECTION".to_string(),
                    e.to_string(),
                )),
            ),
            e @ DSProtocolCatalogErrors::UrnUuidSchema(..) => (
                StatusCode::BAD_REQUEST,
                Json(CatalogErrorOut::new(
                    "400".to_string(),
                    "UUID_SCHEMA".to_string(),
                    e.to_string(),
                )),
            ),
            e @ DSProtocolCatalogErrors::NoMainCatalog => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CatalogErrorOut::new(
                    "500".to_string(),
                    "NO_MAIN_CATALOG".to_string(),
                    e.to_string(),
                )),
            ),
        }
            .into_response()
    }
}
