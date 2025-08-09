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
use crate::provider::core::rainbow_entities::rainbow_catalog_types::{EditCatalogRecordRequest, EditCatalogRequest, NewCatalogRecordRequest, NewCatalogRequest, NewKeywordRequest, NewThemeRequest};
use crate::provider::core::rainbow_entities::RainbowCatalogKeywordsThemesTrait;
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

pub struct RainbowCatalogKeywordsThemesRouter<T>
where 
T: RainbowCatalogKeywordsThemesTrait + Send + Sync + 'static
{
    kt_service: Arc<T>,

}

impl<T> RainbowCatalogKeywordsThemesRouter<T>
where 
T: RainbowCatalogKeywordsThemesTrait + Send + Sync + 'static
 {
    pub fn new(kt_service: Arc<T>) -> Self {
        Self {kt_service}
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/keywords", get(Self::handle_get_all_keywords))
            .route("/api/v1/keywords", get(Self::handle_post_keyword))
            .route("/api/v1/keywords/:keyword_id", get(Self::handle_delete_keyword))
            .route("/api/v1/themes", get(Self::handle_get_all_themes))
            .route("/api/v1/themes", get(Self::handle_post_theme))
            .route("/api/v1/themes/:theme_id", get(Self::handle_delete_theme))
            .with_state(self.kt_service)
    }
    async  fn handle_get_all_keywords(
        State(kt_service): State<Arc<T>>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/keywords");
        match kt_service.get_all_keywords().await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => match e.downcast::<DSProtocolCatalogErrors>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                }
            }
        }
    }async fn handle_post_keyword(
        State(kt_service): State<Arc<T>>,
        input: Result<Json<NewKeywordRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/keywords/");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match kt_service.post_keyword(input).await {
            Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            },
        }
    }
    async fn handle_delete_keyword(
        State(kt_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("Delete /api/v1/keywords/{}", id);
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match kt_service.delete_keyword(id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match  err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
   async  fn handle_get_all_themes(
        State(kt_service): State<Arc<T>>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/themes");
        match kt_service.get_all_themes().await {
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => match e.downcast::<DSProtocolCatalogErrors>() {
                    Ok(e_) => e_.into_response(),
                    Err(e_) => NotCheckedError { inner_error: e_ }.into_response(),
                }
            }
        }
    }async fn handle_post_theme(
        State(kt_service): State<Arc<T>>,
        input: Result<Json<NewThemeRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/themes/");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match kt_service.post_theme(input).await {
            Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            },
        }
    }
    async fn handle_delete_theme(
        State(kt_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("Delete /api/v1/themes/{}", id);
        let id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response()
        };
        match kt_service.delete_theme(id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match  err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}