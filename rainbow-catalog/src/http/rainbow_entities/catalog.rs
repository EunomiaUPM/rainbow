use crate::core::ds_protocol::DSProtocolCatalogTrait;
use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_entities::rainbow_catalog_types::NewCatalogRequest;
use crate::core::rainbow_entities::RainbowCatalogTrait;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
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
            .route("/api/v1/catalogs/:id", get(Self::handle_get_catalogs_by_id))
            .route("/api/v1/catalogs", post(Self::handle_post_catalog))
            .route("/api/v1/catalogs/:id", put(Self::handle_put_catalog))
            .route("/api/v1/catalogs/:id", delete(Self::handle_delete_catalog))
            .with_state((self.catalog_service, self.ds_service))
    }

    async fn handle_get_catalogs(State((catalog_service, ds_service)): State<(Arc<T>, Arc<U>)>) -> impl IntoResponse {
        info!("GET /api/v1/catalogs");

        match ds_service.catalog_request().await {
            Ok(c) => (StatusCode::OK, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
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
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match ds_service.catalog_request_by_id().await {
            Ok(c) => (StatusCode::OK, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_post_catalog(
        State((catalog_service, ds_service)): State<(Arc<T>, Arc<U>)>,
        Json(input): Json<NewCatalogRequest>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/catalogs");
        match catalog_service.post_catalog(input).await {
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
        Json(input): Json<NewCatalogRequest>,
    ) -> impl IntoResponse {
        info!("PUT /api/v1/catalogs/{}", id);
        let catalog_id = match get_urn_from_string(&id) {
            Ok(catalog_id) => catalog_id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match catalog_service.put_catalog(catalog_id, input).await {
            Ok(c) => (StatusCode::ACCEPTED, Json(c)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_delete_catalog(State((catalog_service, ds_service)): State<(Arc<T>, Arc<U>)>, Path(id): Path<String>) -> impl IntoResponse {
        info!("DELETE /api/v1/catalogs/{}", id);
        let catalog_id = match get_urn_from_string(&id) {
            Ok(catalog_id) => catalog_id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
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
