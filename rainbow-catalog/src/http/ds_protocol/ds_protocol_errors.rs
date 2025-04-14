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
        }
            .into_response()
    }
}
