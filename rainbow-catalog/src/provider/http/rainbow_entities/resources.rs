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
use crate::provider::core::rainbow_entities::rainbow_catalog_types::{NewResourceRequest, EditResourceRequest};
use crate::provider::core::rainbow_entities::RainbowCatalogResourceTrait;
use serde::Deserialize;
use anyhow::Error;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::status::InvalidStatusCode;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use rainbow_common::utils::get_urn_from_string;
use serde_json::json;
use std::sync::Arc;
use tracing::info;

pub struct RainbowCatalogResourceRouter<T>
where
T: RainbowCatalogResourceTrait + Send + Sync + 'static
{
    resource_service: Arc<T>,
}

impl<T> RainbowCatalogResourceRouter<T>
where
T: RainbowCatalogResourceTrait + Send + Sync + 'static
{
    pub fn new(resource_service: Arc<T>) -> Self {
        Self {resource_service}
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/resources", get(Self::handle_get_all_resources))
            .route("/api/v1/resources", post(Self::handle_post_resoruce))
            .route("/api/v1/resources/:resource_id", get(Self::handle_get_resoruce_by_id))
            .route("/api/v1/resources/:resource_id", put(Self::handle_put_resoruce_by_id))
            .route("/api/v1/resources/:resource_id", delete(Self::handle_delete_resoruce_by_id))
            .with_state(self.resource_service)
    }

    async fn handle_get_all_resources(
        State(resource_service): State<Arc<T>>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/resources");
        match resource_service.get_all_resources().await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => match e.downcast::<DSProtocolCatalogErrors>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_}.into_response(),
                }
            }
        }
    }
    async fn handle_post_resoruce(
        State(resource_service): State<Arc<T>>,
        input: Result<Json<NewResourceRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/resources");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match resource_service.post_resource(input).await {
            Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            },
        }
    }
    async fn handle_get_resoruce_by_id(
        State(resource_service): State<Arc<T>>,
        Path(resource_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/resources/:resource_id");
        let resource_id = match get_urn_from_string(&resource_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match resource_service.get_resource_by_id(resource_id).await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => match e.downcast::<DSProtocolCatalogErrors>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                }
            }
        }
    }
    async fn handle_put_resoruce_by_id(
        State(resource_service): State<Arc<T>>,
        Path(resource_id): Path<String>,
        input: Result<Json<EditResourceRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("PUT /api/v1/resources/:resource_id");
        let resource_id = match get_urn_from_string(&resource_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),            
        };
        let input = match  input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match resource_service.put_resource_by_id(resource_id, input).await {
            Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
    async fn handle_delete_resoruce_by_id(
        State(resource_service): State<Arc<T>>,
        Path(resource_id): Path<String>,
    ) -> impl IntoResponse {
        info!("DELETE /api/v1/resources/:resource_id");
        let resource_id = match get_urn_from_string(&resource_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match resource_service.delete_resoruce_by_id(resource_id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match  err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}
