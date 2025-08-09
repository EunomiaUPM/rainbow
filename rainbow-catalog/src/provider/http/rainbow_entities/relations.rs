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
use crate::provider::core::rainbow_entities::rainbow_catalog_types::{NewRelationRequest, EditRelationRequest};
use crate::provider::core::rainbow_entities::RainbowRelationsTrait;
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

pub struct RainbowRelationsRouter<T>
where 
T: RainbowRelationsTrait + Send + Sync + 'static
{
    relations_service: Arc<T>,
}

impl<T> RainbowRelationsRouter<T>
where 
T: RainbowRelationsTrait + Send + Sync + 'static
 {
    pub fn new(relations_service: Arc<T>) -> Self {
        Self {relations_service}
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/relations", get(Self::handle_get_relations))
            .route("/api/v1/relations", post(Self::handle_post_relations))
            .route("/api/v1/relations/:relation_id", get(Self::handle_get_relation_by_id))
            .route("/api/v1/relations/:relation_id", put(Self::handle_put_relation_by_id))
            .route("/api/v1/relations/:relation_id", delete(Self::handle_delete_relation_by_id))
            .route("/api/v1/resources/:resource_id/relations", get(Self::handle_get_relations_by_resource))
            .with_state(self.relations_service)
    }
    async fn handle_get_relations(
        State(relations_service): State<Arc<T>>
    ) -> impl IntoResponse {
        info!("GET /api/v1/relations");
        match relations_service.get_relations().await {
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
    async fn handle_post_relations(
        State(relations_service): State<Arc<T>>,
        input: Result<Json<NewRelationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/relations");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match relations_service.post_relation(input).await {
            Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            },
        }
    }
    async fn handle_get_relation_by_id(
        State(relations_service): State<Arc<T>>,
        Path(id): Path<String>
    ) -> impl IntoResponse {
        info!("GET /api/v1/relations/{}", id);
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match relations_service.get_relation_by_id(id).await {
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
    async fn handle_put_relation_by_id(
        State(relations_service): State<Arc<T>>,
        Path(id): Path<String>,
        input: Result<Json<EditRelationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("PUT /api/v1/relations/{}", id);
         let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),            
        };
        let input = match  input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match relations_service.put_relation_by_id(id, input).await {
            Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
    async fn handle_delete_relation_by_id(
        State(relations_service): State<Arc<T>>,
        Path(id): Path<String>
    ) -> impl IntoResponse {
        info!("DELETE /api/v1/relations/{}", id);
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match relations_service.delete_relation(id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match  err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
    async fn handle_get_relations_by_resource(
        State(relations_service): State<Arc<T>>,
        Path(resource_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/resources/{}/relations", resource_id);
        let id = match get_urn_from_string(&resource_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match relations_service.get_relations_from_resource(id).await {
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
}