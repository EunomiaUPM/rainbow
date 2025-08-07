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

use crate::provider::core::ds_protocol::ds_protocol_errors::DSProtocolCatalogErrors;
use crate::provider::core::ds_protocol::DSProtocolCatalogTrait;
use crate::provider::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::{NewCatalogRequest, EditCatalogRequest};
use crate::provider::core::rainbow_entities::RainbowCatalogTrait;
use anyhow::Error;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use rainbow_common::utils::get_urn_from_string;
use std::sync::Arc;
use tracing::info;

pub struct RainbowCatalogCatalogRouter<T, U> {
    catalog_service: Arc<T>,
    ds_service: Arc<U>,
}

impl<T, U> RainbowCatalogCatalogRouter<T, U>
where
    T: RainbowCatalogTrait + Send + Sync + 'static,
    U: DSProtocolCatalogTrait + Send + Sync + 'static,
{
    pub fn new(catalog_service: Arc<T>, ds_service: Arc<U>) -> Self {
        Self { catalog_service, ds_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/catalogs", get(Self::handle_get_catalogs))
            .route("/api/v1/catalogs", post(Self::handle_post_catalog))
            .route("/api/v1/catalogs/:id", get(Self::handle_get_catalogs_by_id))
            .route("/api/v1/catalogs/:id", put(Self::handle_put_catalog))
            .route("/api/v1/catalogs/:id", delete(Self::handle_delete_catalog))
            .route("/api/v1/catalogs/main", post(Self::handle_post_catalog_main))
            .with_state((self.catalog_service, self.ds_service))
    }

    async fn handle_get_catalogs(State((catalog_service, ds_service)): State<(Arc<T>, Arc<U>)>) -> impl IntoResponse {
        info!("GET /api/v1/catalogs");

        match ds_service.catalog_request().await {
            Ok(c) => (StatusCode::OK, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => match e.downcast::<DSProtocolCatalogErrors>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                },
            },
        }
    }

    async fn handle_get_catalogs_by_id(
        State((catalog_service, ds_service)): State<(Arc<T>, Arc<U>)>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/catalogs/{}", id);
        let catalog_id = match get_urn_from_string(&id) {
            Ok(catalog_id) => catalog_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match ds_service.catalog_request_by_id(catalog_id).await {
            Ok(c) => (StatusCode::OK, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => match e.downcast::<DSProtocolCatalogErrors>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                },
            },
        }
    }

    async fn handle_post_catalog(
        State((catalog_service, ds_service)): State<(Arc<T>, Arc<U>)>,
        input: Result<Json<NewCatalogRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/catalogs");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match catalog_service.post_catalog(input, false).await {
            Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_post_catalog_main(
        State((catalog_service, ds_service)): State<(Arc<T>, Arc<U>)>,
        input: Result<Json<NewCatalogRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/catalogs/main");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match catalog_service.post_catalog(input, true).await {
            Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_put_catalog(
        State((catalog_service, ds_service)): State<(Arc<T>, Arc<U>)>,
        Path(id): Path<String>,
        input: Result<Json<EditCatalogRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("PUT /api/v1/catalogs/{}", id);
        let catalog_id = match get_urn_from_string(&id) {
            Ok(catalog_id) => catalog_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match catalog_service.put_catalog(catalog_id, input).await {
            Ok(c) => (StatusCode::ACCEPTED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }


    async fn handle_delete_catalog(
        State((catalog_service, ds_service)): State<(Arc<T>, Arc<U>)>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("DELETE /api/v1/catalogs/{}", id);
        let catalog_id = match get_urn_from_string(&id) {
            Ok(catalog_id) => catalog_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match catalog_service.delete_catalog(catalog_id).await {
            Ok(_) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }
}
