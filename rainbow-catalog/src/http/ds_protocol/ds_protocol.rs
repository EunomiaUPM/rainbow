use crate::core::ds_protocol::ds_protocol_errors::DSProtocolCatalogErrors;
use crate::core::ds_protocol::DSProtocolCatalogTrait;
use anyhow::Error;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use clap::builder::Str;
use rainbow_common::err::transfer_err::TransferErrorType::NotCheckedError;
use rainbow_common::protocol::catalog::catalog_protocol_types::CatalogRequest;
use rainbow_common::utils::get_urn_from_string;
use reqwest::StatusCode;
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

pub struct DSProcotolCatalogRouter<T>
where
    T: DSProtocolCatalogTrait + Send + Sync + 'static,
{
    service: Arc<T>,
}

impl<T> DSProcotolCatalogRouter<T>
where
    T: DSProtocolCatalogTrait + Send + Sync + 'static,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/catalog/request", post(Self::handle_post_catalog_request))
            .route("/catalog/datasets/:dataset_id", get(Self::handle_get_dataset_by_id))
            .with_state(self.service)
    }
    
    async fn handle_post_catalog_request(
        State(service): State<Arc<T>>,
        input: Result<Json<CatalogRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /catalog/request");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return DSProtocolCatalogErrors::JsonRejection(e).into_response(),
        };
        match service.catalog_request().await {
            Ok(catalogs) => (StatusCode::OK, Json(catalogs)).into_response(),
            Err(e) => match e.downcast::<DSProtocolCatalogErrors>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response()
            }
        }
    }

    async fn handle_get_dataset_by_id(
        State(service): State<Arc<T>>,
        Path(dataset_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /catalog/datasets/{}", dataset_id);
        let dataset_id = match get_urn_from_string(&dataset_id) {
            Ok(dataset_id) => dataset_id,
            Err(e) => return DSProtocolCatalogErrors::UrnUuidSchema(e.to_string()).into_response(),
        };
        match service.dataset_request(dataset_id).await {
            Ok(dataset) => (StatusCode::OK, Json(dataset)).into_response(),
            Err(e) => match e.downcast::<DSProtocolCatalogErrors>() {
                Ok(e_) => e_.into_response(),
                Err(e_) => NotCheckedError { inner_error: e_ }.into_response()
            }
        }
    }
}
