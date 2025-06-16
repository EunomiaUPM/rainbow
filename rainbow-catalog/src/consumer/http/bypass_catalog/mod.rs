use crate::consumer::core::bypass_service::ByPassTrait;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use rainbow_common::utils::get_urn_from_string;
use reqwest::{Client, StatusCode};
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

pub struct CatalogBypassRouter<T>
where
    T: ByPassTrait + Send + Sync + 'static,
{
    service: Arc<T>,
}

impl<T> CatalogBypassRouter<T>
where
    T: ByPassTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/catalog-bypass/:participant_id/*extra",
                get(Self::forward_to_catalog),
            )
            .with_state(self.service)
    }
    async fn forward_to_catalog(
        State(service): State<Arc<T>>,
        Path((participant_id, extra)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/catalog-bypass/{}/{}", participant_id, extra);
        let participant_id = match get_urn_from_string(&participant_id) {
            Ok(participant_id) => participant_id,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match service.bypass(participant_id, extra).await {
            Ok(value) => (StatusCode::OK, Json(value)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}
