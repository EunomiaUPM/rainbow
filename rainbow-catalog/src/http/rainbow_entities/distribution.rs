use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_entities::rainbow_catalog_types::{EditDistributionRequest, NewDatasetRequest, NewDistributionRequest};
use crate::core::rainbow_entities::RainbowDistributionTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::utils::get_urn_from_string;
use reqwest::StatusCode;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

pub struct RainbowCatalogDistributionRouter<T> {
    distribution_service: Arc<T>,
}

impl<T> RainbowCatalogDistributionRouter<T>
where
    T: RainbowDistributionTrait + Send + Sync + 'static,
{
    pub fn new(distribution_service: Arc<T>) -> Self {
        Self { distribution_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/distributions/:id",
                get(Self::handle_get_distributions_by_id),
            )
            .route(
                "/api/v1/datasets/:id/distributions",
                get(Self::handle_get_distributions_by_dataset_id),
            )
            .route(
                "/api/v1/catalogs/:id/datasets/:did/distributions",
                post(Self::handle_post_distribution),
            )
            .route(
                "/api/v1/catalogs/:id/datasets/:did/distributions/:ddid",
                put(Self::handle_put_distribution),
            )
            .route(
                "/api/v1/catalogs/:id/datasets/:did/distributions/:ddid",
                delete(Self::handle_delete_distribution),
            )
            .with_state(self.distribution_service)
    }

    async fn handle_get_distributions_by_id(
        State(distribution_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/distributions/:id");
        let distribution_id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match distribution_service.get_distribution_by_id(distribution_id).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_get_distributions_by_dataset_id(
        State(distribution_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datasets/{}/distributions", id);
        let dataset_id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match distribution_service.get_distributions_by_dataset_id(dataset_id).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_post_distribution(
        State(distribution_service): State<Arc<T>>,
        Path((id, did)): Path<(String, String)>,
        input: Result<Json<NewDistributionRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/catalogs/{}/distributions", id);
        let catalog_id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let distribution_id = match get_urn_from_string(&did) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match distribution_service.post_distribution(catalog_id, distribution_id, input).await {
            Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_put_distribution(
        State(distribution_service): State<Arc<T>>,
        Path((c_id, d_id, ds_id)): Path<(String, String, String)>,
        input: Result<Json<EditDistributionRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "PUT /api/v1/catalogs/{}/datasets/{}/distributions/{}",
            c_id, d_id, ds_id
        );
        let catalog_id = match get_urn_from_string(&c_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let dataset_id = match get_urn_from_string(&d_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let distribution_id = match get_urn_from_string(&ds_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return CatalogError::JsonRejection(e).into_response(),
        };
        match distribution_service.put_distribution(catalog_id, dataset_id, distribution_id, input).await {
            Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_delete_distribution(
        State(distribution_service): State<Arc<T>>,
        Path((c_id, d_id, ds_id)): Path<(String, String, String)>,
    ) -> impl IntoResponse {
        info!(
            "DELETE /api/v1/catalogs/{}/datasets/{}/distributions/{}",
            c_id, d_id, ds_id
        );
        let catalog_id = match get_urn_from_string(&c_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let dataset_id = match get_urn_from_string(&d_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        let distribution_id = match get_urn_from_string(&ds_id) {
            Ok(id) => id,
            Err(err) => return CatalogError::UrnUuidSchema(err.to_string()).into_response(),
        };
        match distribution_service.delete_distribution(catalog_id, dataset_id, distribution_id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }
}
