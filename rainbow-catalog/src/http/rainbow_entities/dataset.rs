use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_entities::rainbow_catalog_types::NewDatasetRequest;
use crate::core::rainbow_entities::RainbowDatasetTrait;
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

pub struct RainbowCatalogDatasetRouter<T> {
    dataset_service: Arc<T>,
}

impl<T> RainbowCatalogDatasetRouter<T>
where
    T: RainbowDatasetTrait + Send + Sync + 'static,
{
    pub fn new(dataset_service: Arc<T>) -> Self {
        Self { dataset_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/datasets/:id", get(Self::handle_get_dataset_by_id))
            .route(
                "/api/v1/catalogs/:id/datasets",
                post(Self::handle_post_dataset),
            )
            .route(
                "/api/v1/catalogs/:id/datasets/:did",
                put(Self::handle_put_dataset),
            )
            .route(
                "/api/v1/catalogs/:id/datasets/:did",
                delete(Self::handle_delete_dataset),
            )
            .with_state(self.dataset_service)
    }

    async fn handle_get_dataset_by_id(
        State(dataset_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datasets/:id");
        let dataset_id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match dataset_service.get_dataset_by_id(dataset_id).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_post_dataset(
        State(dataset_service): State<Arc<T>>,
        Path(id): Path<String>,
        Json(input): Json<NewDatasetRequest>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/catalogs/{}/datasets", id);
        let dataset_id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match dataset_service.post_dataset(dataset_id, input).await {
            Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_put_dataset(
        State(dataset_service): State<Arc<T>>,
        Path((c_id, d_id)): Path<(String, String)>,
        Json(input): Json<NewDatasetRequest>,
    ) -> impl IntoResponse {
        info!("PUT /api/v1/catalogs/{}/datasets/{}", c_id, d_id);
        let catalog_id = match get_urn_from_string(&c_id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let dataset_id = match get_urn_from_string(&d_id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match dataset_service.put_dataset(catalog_id, dataset_id, input).await {
            Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_delete_dataset(
        State(dataset_service): State<Arc<T>>,
        Path((c_id, d_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!("DELETE /api/v1/catalogs/{}/datasets/{}", c_id, d_id);
        let catalog_id = match get_urn_from_string(&c_id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let dataset_id = match get_urn_from_string(&d_id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match dataset_service.delete_dataset(catalog_id, dataset_id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }
}
