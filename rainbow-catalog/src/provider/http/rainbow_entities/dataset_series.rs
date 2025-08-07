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
use crate::provider::core::rainbow_entities::rainbow_catalog_types::{NewDatasetSeriesRequest, EditDatasetSeriesRequest};
use crate::provider::core::rainbow_entities::RainbowCatalogDatasetSeriesTrait;
use rainbow_db::catalog::entities::dataset;
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

pub struct RainbowCatalogDatasetSeriesRouter<T>
where 
T: RainbowCatalogDatasetSeriesTrait + Send + Sync + 'static
{
    dataset_series_service: Arc<T>,

}

impl<T> RainbowCatalogDatasetSeriesRouter<T>
where
T: RainbowCatalogDatasetSeriesTrait + Send + Sync + 'static
{
    pub fn new(dataset_series_service: Arc<T>) -> Self {
        Self {dataset_series_service}
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/dataset_series", get(Self::handle_get_dataset_seriers))
            .route("/api/v1/dataset_series", post(Self::handle_post_dataset_seriers))
            .route("/api/v1/dataset_series/:dataset_series_id", get(Self::handle_get_dataset_seriers_by_id))
            .route("/api/v1/dataset_series/:dataset_series_id", put(Self::handle_put_dataset_seriers))
            .route("/api/v1/dataset_series/:dataset_series_id", delete(Self::handle_delete_dataset_seriers))
            .route("/api/v1/dataset_series/:dataset_series_id/datasets", get(Self::handle_get_datasets_from_dataset_seriers))
            .with_state(self.dataset_series_service)
    }
    async fn handle_get_dataset_seriers(
        State(dataset_series_service): State<Arc<T>>,
    ) -> impl IntoResponse {
        info!( "GET /api/v1/dataset_series");
        match dataset_series_service.get_all_dataset_series().await {
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
    async fn handle_post_dataset_seriers(
        State(dataset_series_service): State<Arc<T>>,
        input: Result<Json<NewDatasetSeriesRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/dataset_series");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match dataset_series_service.post_dataset_series(input).await {
            Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            },
        }     
    }
    async fn handle_get_dataset_seriers_by_id(
        State(dataset_series_service): State<Arc<T>>,
        Path(id): Path<String>
    ) -> impl IntoResponse {
        info!("GET /api/v1/dataset_series/{}", id);
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match dataset_series_service.get_dataset_series_by_id(id).await {
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
    async fn handle_put_dataset_seriers(
        State(dataset_series_service): State<Arc<T>>,
        Path(id): Path<String>,
        input: Result<Json<EditDatasetSeriesRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("/api/v1/dataset_series/{}", id);
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),            
        };
        let input = match  input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match dataset_series_service.put_dataset_series_by_id(id, input).await {
            Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }     
    }
    async fn handle_delete_dataset_seriers(
        State(dataset_series_service): State<Arc<T>>,
        Path(id): Path<String>
    ) -> impl IntoResponse {
        info!("/api/v1/dataset_series/{}", id); 
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match dataset_series_service.delete_dataset_series_by_id(id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match  err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }      
    }
    async fn handle_get_datasets_from_dataset_seriers(
        State(dataset_series_service): State<Arc<T>>,
        Path(id): Path<String>
    ) -> impl IntoResponse {
        info!("/api/v1/dataset_series/{}/datasets", id);
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match dataset_series_service.get_datasets_from_dataset_series_by_id(id).await {
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