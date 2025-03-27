use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_entities::rainbow_catalog_types::{EditDataServiceRequest, NewDataServiceRequest};
use crate::core::rainbow_entities::RainbowDataServiceTrait;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::utils::get_urn_from_string;
use std::sync::Arc;
use tracing::info;

pub struct RainbowCatalogDataServiceRouter<T> {
    data_service_service: Arc<T>,
}

impl<T> RainbowCatalogDataServiceRouter<T>
where
    T: RainbowDataServiceTrait + Send + Sync + 'static,
{
    pub fn new(data_service_service: Arc<T>) -> Self {
        Self { data_service_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/data-services/:id",
                get(Self::handle_get_data_service_by_id),
            )
            .route(
                "/api/v1/catalogs/:id/data-services",
                post(Self::handle_post_data_service),
            )
            .route(
                "/api/v1/catalogs/:id/data-services/:did",
                put(Self::handle_put_data_service),
            )
            .route(
                "/api/v1/catalogs/:id/data-services/:did",
                delete(Self::handle_delete_data_service),
            )
            .with_state(self.data_service_service)
    }
    async fn handle_get_data_service_by_id(
        State(data_service_service): State<Arc<T>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/data-services/:id");
        let dataservice_id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match data_service_service.get_data_service_by_id(dataservice_id).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_post_data_service(
        State(data_service_service): State<Arc<T>>,
        Path(id): Path<String>,
        Json(input): Json<NewDataServiceRequest>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/catalogs/{}/data-services", id);
        let dataservice_id = match get_urn_from_string(&id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match data_service_service.post_data_service(dataservice_id, input).await {
            Ok(d) => (StatusCode::CREATED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_put_data_service(
        State(data_service_service): State<Arc<T>>,
        Path((c_id, ds_id)): Path<(String, String)>,
        Json(input): Json<EditDataServiceRequest>,
    ) -> impl IntoResponse {
        info!("PUT /api/v1/catalogs/{}/data-services/{}", c_id, ds_id);
        let catalog_id = match get_urn_from_string(&c_id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let dataservice_id = match get_urn_from_string(&ds_id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match data_service_service.put_data_service(catalog_id, dataservice_id, input).await {
            Ok(d) => (StatusCode::ACCEPTED, Json(d)).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }

    async fn handle_delete_data_service(
        State(data_service_service): State<Arc<T>>,
        Path((c_id, ds_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!("DELETE /api/v1/catalogs/{}/data-services/{}", c_id, ds_id);
        let catalog_id = match get_urn_from_string(&c_id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        let dataservice_id = match get_urn_from_string(&ds_id) {
            Ok(id) => id,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };
        match data_service_service.delete_data_service(catalog_id, dataservice_id).await {
            Ok(d) => (StatusCode::ACCEPTED).into_response(),
            Err(err) => match err.downcast::<CatalogError>() {
                Ok(e) => e.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            },
        }
    }
}
