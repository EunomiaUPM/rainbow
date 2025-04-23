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
use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_entities::rainbow_catalog_types::EditDistributionRequest;
use crate::core::rainbow_entities::{RainbowDistributionTrait, RainbowPoliciesTrait};
use crate::protocol::policies::OdrlPolicy;
use anyhow::bail;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use log::info;
use rainbow_common::utils::get_urn_from_string;
use reqwest::StatusCode;
use serde_json::Value;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub struct RainbowCatalogPoliciesRouter<T> {
    policies_service: Arc<T>,
}

impl<T> RainbowCatalogPoliciesRouter<T>
where
    T: RainbowPoliciesTrait + Send + Sync + 'static,
{
    pub fn new(policies_service: Arc<T>) -> Self {
        Self { policies_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/catalogs/:id/policies",
                get(Self::handle_get_catalog_policies),
            )
            .route(
                "/api/v1/catalogs/:id/policies",
                post(Self::handle_post_catalog_policies),
            )
            .route(
                "/api/v1/catalogs/:id/policies/:pid",
                delete(Self::handle_delete_catalog_policies),
            )
            // DATASET POLICIES
            .route(
                "/api/v1/datasets/:id/policies",
                get(Self::handle_get_dataset_policies),
            )
            .route(
                "/api/v1/datasets/:id/policies",
                post(Self::handle_post_dataset_policies),
            )
            .route(
                "/api/v1/datasets/:id/policies/:pid",
                delete(Self::handle_delete_dataset_policies),
            )
            // DATA SERVICES POLICIES
            .route(
                "/api/v1/data-services/:id/policies",
                get(Self::handle_get_dataservices_policies),
            )
            .route(
                "/api/v1/data-services/:id/policies",
                post(Self::handle_post_dataservices_policies),
            )
            .route(
                "/api/v1/data-services/:id/policies/:pid",
                delete(Self::handle_delete_dataservices_policies),
            )
            // DISTRIBUTION POLICIES
            .route(
                "/api/v1/distributions/:id/policies",
                get(Self::handle_get_distributions_policies),
            )
            .route(
                "/api/v1/distributions/:id/policies",
                post(Self::handle_post_distributions_policies),
            )
            .route(
                "/api/v1/distributions/:id/policies/:pid",
                delete(Self::handle_delete_distributions_policies),
            )
            .with_state(self.policies_service)
    }

    async fn handle_get_catalog_policies(
        State(policies_service): State<Arc<T>>,
        Path(catalog_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/catalogs/{}/policies", catalog_id);
        let catalog_id = match get_urn_from_string(&catalog_id) {
            Ok(catalog_id) => catalog_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };

        match policies_service.get_catalog_policies(catalog_id).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_post_catalog_policies(
        State(policies_service): State<Arc<T>>,
        Path(catalog_id): Path<String>,
        input: Result<Json<OdrlPolicy>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/catalogs/{}/policies", catalog_id);
        let catalog_id = match get_urn_from_string(&catalog_id) {
            Ok(catalog_id) => catalog_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match policies_service.post_catalog_policies(catalog_id, input).await {
            Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_delete_catalog_policies(
        State(policies_service): State<Arc<T>>,
        Path((catalog_id, policy_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!(
            "DELETE /api/v1/catalogs/{}/policies/{}",
            catalog_id, policy_id
        );
        let catalog_id = match get_urn_from_string(&catalog_id) {
            Ok(catalog_id) => catalog_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let policy_id = match get_urn_from_string(&policy_id) {
            Ok(policy_id) => policy_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };

        match policies_service.delete_catalog_policies(catalog_id, policy_id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_get_dataset_policies(
        State(policies_service): State<Arc<T>>,
        Path(dataset_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datasets/{}/policies", dataset_id);
        let dataset_id = match get_urn_from_string(&dataset_id) {
            Ok(dataset_id) => dataset_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match policies_service.get_dataset_policies(dataset_id).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_post_dataset_policies(
        State(policies_service): State<Arc<T>>,
        Path(dataset_id): Path<String>,
        input: Result<Json<OdrlPolicy>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/datasets/{}/policies", dataset_id);
        let dataset_id = match get_urn_from_string(&dataset_id) {
            Ok(dataset_id) => dataset_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match policies_service.post_dataset_policies(dataset_id, input).await {
            Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_delete_dataset_policies(
        State(policies_service): State<Arc<T>>,
        Path((dataset_id, policy_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!(
            "DELETE /api/v1/datasets/{}/policies/{}",
            dataset_id, policy_id
        );
        let dataset_id = match get_urn_from_string(&dataset_id) {
            Ok(dataset_id) => dataset_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let policy_id = match get_urn_from_string(&policy_id) {
            Ok(policy_id) => policy_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match policies_service.delete_dataset_policies(dataset_id, policy_id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_get_dataservices_policies(
        State(policies_service): State<Arc<T>>,
        Path(data_service_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/data-services/{}/policies", data_service_id);
        let dataservice_id = match get_urn_from_string(&data_service_id) {
            Ok(dataservice_id) => dataservice_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match policies_service.get_data_service_policies(dataservice_id).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_post_dataservices_policies(
        State(policies_service): State<Arc<T>>,
        Path(data_service_id): Path<String>,
        input: Result<Json<OdrlPolicy>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/data-services/{}/policies", data_service_id);
        let dataservice_id = match get_urn_from_string(&data_service_id) {
            Ok(dataservice_id) => dataservice_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match policies_service.post_data_service_policies(dataservice_id, input).await {
            Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_delete_dataservices_policies(
        State(policies_service): State<Arc<T>>,
        Path((dataservice_id, policy_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!(
            "DELETE /api/v1/data-services/{}/policies/{}",
            dataservice_id, policy_id
        );
        let dataservice_id = match get_urn_from_string(&dataservice_id) {
            Ok(dataservice_id) => dataservice_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let policy_id = match get_urn_from_string(&policy_id) {
            Ok(policy_id) => policy_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match policies_service.delete_data_service_policies(dataservice_id, policy_id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_get_distributions_policies(
        State(policies_service): State<Arc<T>>,
        Path(distribution_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/distributions/{}/policies", distribution_id);
        let distribution_id = match get_urn_from_string(&distribution_id) {
            Ok(distribution_id) => distribution_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match policies_service.get_distribution_policies(distribution_id).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_post_distributions_policies(
        State(policies_service): State<Arc<T>>,
        Path(distribution_id): Path<String>,
        input: Result<Json<OdrlPolicy>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "POST /api/v1/distributions/{}/policies",
            distribution_id.to_string()
        );
        let distribution_id = match get_urn_from_string(&distribution_id) {
            Ok(distribution_id) => distribution_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match policies_service.post_distribution_policies(distribution_id, input).await {
            Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_delete_distributions_policies(
        State(policies_service): State<Arc<T>>,
        Path((distribution_id, policy_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!(
            "DELETE /api/v1/distributions/{}/policies/{}",
            distribution_id, policy_id
        );
        let distribution_id = match get_urn_from_string(&distribution_id) {
            Ok(distribution_id) => distribution_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let policy_id = match get_urn_from_string(&policy_id) {
            Ok(policy_id) => policy_id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match policies_service.delete_distribution_policies(distribution_id, policy_id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }
}
