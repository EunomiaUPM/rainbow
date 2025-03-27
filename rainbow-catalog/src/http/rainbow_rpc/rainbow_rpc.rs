use crate::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::core::rainbow_rpc::rainbow_rpc_types::RainbowRPCCatalogResolveDataServiceRequest;
use crate::core::rainbow_rpc::RainbowRPCCatalogTrait;
use anyhow::Error;
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::info;

pub struct RainbowRPCCatalogRouter<T> {
    service: Arc<T>,
}

impl<T> RainbowRPCCatalogRouter<T>
where
    T: RainbowRPCCatalogTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route(
                "/api/v1/catalog/rpc/resolve-data-service",
                post(Self::handle_resolve_data_service),
            )
            .with_state(self.service)
    }
    pub async fn handle_resolve_data_service(
        State(service): State<Arc<T>>,
        input: Result<Json<RainbowRPCCatalogResolveDataServiceRequest>, JsonRejection>, // Todo define object
    ) -> impl IntoResponse {
        info!("POST /api/v1/catalog/rpc/resolve-data-service");
        match service.resolve_data_service(input.unwrap().0).await {
            Ok(d) => (StatusCode::OK, Json(d)).into_response(),
            Err(e) => match e.downcast::<CatalogError>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => (StatusCode::INTERNAL_SERVER_ERROR, NotCheckedError { inner_error: e_ }).into_response()
            }
        }
    }
}
