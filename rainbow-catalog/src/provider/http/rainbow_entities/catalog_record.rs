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
use crate::provider::core::rainbow_entities::rainbow_catalog_types::{EditCatalogRecordRequest, EditCatalogRequest, NewCatalogRecordRequest, NewCatalogRequest};
use crate::provider::core::rainbow_entities::RainbowCatalogRecrodTrait;
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

pub struct RainbowCatalogRecordRouter<T>
where 
T: RainbowCatalogRecrodTrait + Send + Sync + 'static
{
    catalog_record_service: Arc<T>,

}

impl<T> RainbowCatalogRecordRouter<T>
where 
T: RainbowCatalogRecrodTrait + Send + Sync + 'static
 {
    pub fn new(catalog_record_service: Arc<T>) -> Self {
        Self {catalog_record_service}
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/catalog_records", get(Self::handle_get_catalog_records))
            .route("/api/v1/catalog_records", post(Self::handle_post_catalog_record))
            .route("/api/v1/catalog_records/:catalog_record_id", get(Self::handle_get_catalog_record_by_id))
            .route("/api/v1/catalog_records/:catalog_record_id", put(Self::handle_put_catalog_record_by_id))
            .route("/api/v1/catalog_records/:catalog_record_id", delete(Self::handle_delete_catalog_record_by_id))
            .route("/api/v1/catalogs/:catalogid/catalog_records", get(Self::handle_get_catalog_records_by_catalog))
            .with_state(self.catalog_record_service)
    }
    async fn handle_get_catalog_records(
        State(catalog_record_service): State<Arc<T>>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/catalog_records");
        match catalog_record_service.get_catalog_records().await {
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
    async fn handle_get_catalog_records_by_catalog(
        State(catalog_record_service): State<Arc<T>>,
        Path(catalog_id): Path<String>
    ) -> impl IntoResponse {
        info!("GET /api/v1/catalogs/{}/catalog_records", catalog_id);
        let catalog_id = match get_urn_from_string(&catalog_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match catalog_record_service.get_catalog_records_by_catalog(catalog_id).await {
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
    async fn handle_post_catalog_record(
        State(catalog_record_service): State<Arc<T>>,
        input: Result<Json<NewCatalogRecordRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/catalog_records"); 
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match catalog_record_service.post_catalog_record(input).await {
            Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            },
        }
    }
    async fn handle_get_catalog_record_by_id(
        State(catalog_record_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/catalog_records/{}", id);
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match catalog_record_service.get_catalog_records_by_id(id).await {
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
    async fn handle_put_catalog_record_by_id(
        State(catalog_record_service): State<Arc<T>>,
        Path(catalog_record_id): Path<String>,
        input: Result<Json<EditCatalogRecordRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("PUT /api/v1/catalog_records/{}", catalog_record_id);  
        let catalog_record_id = match get_urn_from_string(&catalog_record_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),            
        };
        let input = match  input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match catalog_record_service.put_catalog_record_by_id(catalog_record_id, input).await {
            Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
    async fn handle_delete_catalog_record_by_id(
        State(catalog_record_service): State<Arc<T>>,
        Path(catalog_record_id): Path<String>
    ) -> impl IntoResponse {
        info!("DELETE /api/v1/catalog_records/{}", catalog_record_id); 
        let catalog_record_id = match get_urn_from_string(&catalog_record_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match catalog_record_service.delete_catalog_record_by_id(catalog_record_id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match  err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}