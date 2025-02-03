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

use crate::core::idsa_api::{catalog_request, dataset_request, CatalogRequestMessage};
use axum::extract::rejection::{JsonRejection, PathRejection};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn catalog_router() -> anyhow::Result<Router> {
    let router = Router::new()
        .route("/catalog/request", post(handle_catalog_request))
        .route("/catalog/datasets/:id", get(handle_get_dataset));
    Ok(router)
}

async fn handle_catalog_request(
    result: Result<Json<CatalogRequestMessage>, JsonRejection>,
) -> impl IntoResponse {
    info!("POST /catalog/request");

    match result {
        Ok(Json(input)) => match catalog_request().await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        },
        Err(err) => match err {
            JsonRejection::JsonDataError(_) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
            _ => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        },
    }
}

async fn handle_get_dataset(result: Result<Path<String>, PathRejection>) -> impl IntoResponse {
    info!("GET /catalog/datasets/:id");

    match result {
        Ok(id) => match dataset_request(id.0).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(err) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
        },
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response()
    }
}
