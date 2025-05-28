// use crate::core::datahub_proxy::datahub_proxy_types::{DatasetsQueryOptions, DomainsQueryOptions};
use crate::core::datahub_proxy::datahub_proxy_types::{DomainsQueryOptions};
use crate::core::datahub_proxy::DatahubProxyTrait;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::info;

pub struct DataHubProxyRouter<T>
where
    T: DatahubProxyTrait + Send + Sync + 'static,
{
    datahub_service: Arc<T>,
}


impl<T> DataHubProxyRouter<T>
where
    T: DatahubProxyTrait + Send + Sync + 'static,
{
    pub fn new(datahub_service: Arc<T>) -> Self {
        Self {
            datahub_service
        }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/datahub/domains", get(Self::handle_get_datahub_domains))
            // .route("/api/v1/datahub/domains/:domain_id", get(Self::handle_get_datahub_domain_by_id))
            // .route("/api/v1/datahub/domains/:domain_id/datasets", get(Self::handle_get_datasets_by_domain_id))
            // .route("/api/v1/datahub/domains/:domain_id/datasets/:dataset_id", get(Self::handle_get_datasets_by_id))
            .with_state(self.datahub_service)
    }
    async fn handle_get_datahub_domains(
        State(datahub_service): State<Arc<T>>,
        query: Query<DomainsQueryOptions>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/datahub/domains");
        match datahub_service.get_datahub_domains().await {
            Ok(domains) => (StatusCode::OK, Json(domains)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
    // async fn handle_get_datahub_domain_by_id(
    //     State(datahub_service): State<Arc<T>>,
    //     Path(domain_id): Path<String>,
    // ) -> impl IntoResponse {
    //     info!("GET /api/v1/datahub/domains/{}", domain_id);
    //     match datahub_service.get_datahub_domain_by_id(domain_id).await {
    //         Ok(domain) => (StatusCode::OK, Json(domain)).into_response(),
    //         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    //     }
    // }
    // async fn handle_get_datasets_by_domain_id(
    //     State(datahub_service): State<Arc<T>>,
    //     Path(domain_id): Path<String>,
    //     query: Query<DatasetsQueryOptions>,
    // ) -> impl IntoResponse {
    //     info!("GET /api/v1/datahub/domains/{}/datasets", domain_id);
    //     match datahub_service.get_datahub_datasets_by_domain_id(domain_id).await {
    //         Ok(datasets) => (StatusCode::OK, Json(datasets)).into_response(),
    //         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    //     }
    // }
    // async fn handle_get_datasets_by_id(
    //     State(datahub_service): State<Arc<T>>,
    //     Path((domain_id, dataset_id)): Path<(String, String)>,
    // ) -> impl IntoResponse {
    //     info!("GET /api/v1/datahub/domains/{}/datasets/{}", domain_id, dataset_id);
    //     match datahub_service.get_datahub_dataset_by_id(dataset_id).await {
    //         Ok(dataset) => (StatusCode::OK, Json(dataset)).into_response(),
    //         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    //     }
    // }
}