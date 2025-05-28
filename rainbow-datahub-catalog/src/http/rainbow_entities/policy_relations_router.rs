// use crate::core::datahub_proxy::datahub_proxy_types::{DatasetsQueryOptions, DomainsQueryOptions};
// use crate::core::datahub_proxy::DatahubProxyTrait;
// use crate::core::rainbow_entities::PolicyTemplatesToDatahubDatasetRelationTrait;
// use axum::extract::{Path, Query, State};
// use axum::response::IntoResponse;
// use axum::routing::get;
// use axum::{Json, Router};
// use reqwest::StatusCode;
// use std::sync::Arc;
// use tracing::info;

// pub struct RainbowDatahubPolicyRelationsRouter<T, U>
// where
//     T: DatahubProxyTrait + Send + Sync + 'static,
//     U: PolicyTemplatesToDatahubDatasetRelationTrait + Send + Sync + 'static,
// {
//     datahub_service: Arc<T>,
//     policy_relations_service: Arc<U>,
// }


// impl<T, U> RainbowDatahubPolicyRelationsRouter<T, U>
// where
//     T: DatahubProxyTrait + Send + Sync,
//     U: PolicyTemplatesToDatahubDatasetRelationTrait + Send + Sync,
// {
//     pub fn new(datahub_service: Arc<T>, policy_relations_service: Arc<U>) -> Self {
//         Self {
//             datahub_service,
//             policy_relations_service,
//         }
//     }
//     pub fn router(self) -> Router {
//         Router::new()
//             .route("/api/v1/datahub/hola-pablo", get(Self::handle_your_routes_here))
//             .with_state((self.datahub_service, self.policy_relations_service))
//     }
//     async fn handle_your_routes_here(
//         State((datahub_service, policy_relations_service)): State<(Arc<T>, Arc<U>)>,
//     ) -> impl IntoResponse {
//         info!("GET /api/v1/datahub/hola-pablo");
//         match datahub_service.get_datahub_domains().await {
//             Ok(domains) => (StatusCode::OK, Json(domains)).into_response(),
//             Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
//         }
//     }
// }