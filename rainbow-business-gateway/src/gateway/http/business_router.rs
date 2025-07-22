use crate::gateway::core::business::BusinessCatalogTrait;
use crate::gateway::http::business_router_types::{RainbowBusinessAcceptanceRequest, RainbowBusinessNegotiationRequest, RainbowBusinessTerminationRequest};
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{middleware, Extension, Json, Router};
use rainbow_common::auth::business::RainbowBusinessLoginRequest;
use rainbow_common::auth::header::{extract_request_info, RequestInfo};
use rainbow_common::protocol::contract::contract_odrl::OdrlPolicyInfo;
use rainbow_common::utils::get_urn_from_string;
use serde_json::json;
use std::sync::Arc;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tracing::info;

pub struct RainbowBusinessRouter<T>
where
    T: BusinessCatalogTrait + Sync + Send,
{
    service: Arc<T>,
}
impl<T> RainbowBusinessRouter<T>
where
    T: BusinessCatalogTrait + Sync + Send,
{
    pub fn new(service: Arc<T>) -> Self {
        Self { service }
    }
    pub fn router(self) -> Router {
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(Any)
            .allow_headers(AllowHeaders::list([CONTENT_TYPE, AUTHORIZATION]));

        Router::new()
            // Common
            .route(
                "/gateway/api/catalogs",
                get(Self::handle_business_get_catalogs),
            )
            .route(
                "/gateway/api/catalogs/:catalog_id/datasets",
                get(Self::handle_business_get_datasets_by_catalog),
            )
            .route(
                "/gateway/api/catalogs/datasets/:dataset_id",
                get(Self::handle_business_get_dataset),
            )
            .route(
                "/gateway/api/catalogs/:catalog_id/datasets/:dataset_id/policies",
                get(Self::handle_business_get_policy_offers_by_dataset),
            )
            .route("/gateway/api/login", post(Self::handle_login))
            .route("/gateway/api/login/poll", post(Self::handle_login_poll))
            .route("/gateway/api/negotiation/rpc/terminate", post(Self::handle_terminate_request))
            // Business User
            .route(
                "/gateway/api/policy-templates",
                get(Self::handle_business_get_policy_templates),
            )
            .route(
                "/gateway/api/policy-templates/:pt_id",
                get(Self::handle_business_get_policy_template_by_id),
            )
            .route(
                "/gateway/api/catalogs/:catalog_id/datasets/:dataset_id/policies",
                post(Self::handle_business_post_policy_offer),
            )
            .route(
                "/gateway/api/catalogs/:catalog_id/datasets/:dataset_id/policies/:policy_id",
                delete(Self::handle_business_delete_policy_offer),
            )
            .route(
                "/gateway/api/negotiation/business/requests",
                get(Self::handle_get_business_negotiation_requests),
            )
            .route(
                "/gateway/api/negotiation/business/requests/:request_id",
                get(Self::handle_get_business_negotiation_request_by_id),
            )
            .route(
                "/gateway/api/negotiation/rpc/accept",
                post(Self::handle_accept_request),
            )
            // Customer
            .route(
                "/gateway/api/negotiation/consumer/:participant_id/requests",
                get(Self::handle_get_customer_negotiation_requests),
            )
            .route(
                "/gateway/api/negotiation/consumer/:participant_id/requests/:request_id",
                get(Self::handle_get_consumer_negotiation_request_by_id),
            )
            .route(
                "/gateway/api/negotiation/rpc/request",
                post(Self::handle_create_request),
            )
            // Others
            .layer(middleware::from_fn(extract_request_info))
            .layer(cors)
            .with_state(self.service)
    }
    async fn handle_business_get_catalogs(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
    ) -> impl IntoResponse {
        info!("GET /gateway/api/catalogs");
        let token = &info.token;

        match service.get_catalogs(token.to_string()).await {
            Ok(catalogs) => (StatusCode::OK, Json(catalogs)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_business_get_datasets_by_catalog(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        Path(catalog_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /gateway/api/catalogs/{}/datasets", catalog_id);
        let token = &info.token;
        let catalog_id = match get_urn_from_string(&catalog_id) {
            Ok(catalog_id) => catalog_id,
            Err(_err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "urn not serializable"})),
                )
                    .into_response()
            }
        };

        match service.get_datasets_by_catalog(catalog_id, token.to_string()).await {
            Ok(datasets) => (StatusCode::OK, Json(datasets)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_business_get_dataset(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        Path(dataset_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /gateway/api/catalogs/datasets/{}", dataset_id);
        let token = &info.token;
        let dataset_id = match get_urn_from_string(&dataset_id) {
            Ok(dataset_id) => dataset_id,
            Err(_err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "urn not serializable"})),
                )
                    .into_response()
            }
        };
        match service.get_dataset(dataset_id, token.to_string()).await {
            Ok(dataset) => (StatusCode::OK, Json(dataset)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_business_get_policy_templates(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
    ) -> impl IntoResponse {
        info!("GET /gateway/api/policy-templates");
        let token = &info.token;

        match service.get_policy_templates(token.to_string()).await {
            Ok(policy_templates) => (StatusCode::OK, Json(policy_templates)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_business_get_policy_template_by_id(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        Path(pt_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /gateway/api/policy-templates/{}", pt_id);
        let token = &info.token;

        match service.get_policy_template_by_id(pt_id, token.to_string()).await {
            Ok(policy_template) => (StatusCode::OK, Json(policy_template)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_business_get_policy_offers_by_dataset(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        Path((catalog_id, dataset_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!(
            "GET /gateway/api/catalogs/{}/datasets/{}/policies",
            catalog_id, dataset_id
        );
        let token = &info.token;
        let dataset_id = match get_urn_from_string(&dataset_id) {
            Ok(dataset_id) => dataset_id,
            Err(_err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "urn not serializable"})),
                )
                    .into_response()
            }
        };

        match service.get_policy_offers_by_dataset(dataset_id, token.to_string()).await {
            Ok(offers) => (StatusCode::OK, Json(offers)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_business_post_policy_offer(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        Path((catalog_id, dataset_id)): Path<(String, String)>,
        input: Result<Json<OdrlPolicyInfo>, JsonRejection>,
    ) -> impl IntoResponse {
        info!(
            "POST /gateway/api/catalogs/{}/datasets/{}/policies",
            catalog_id, dataset_id
        );
        let token = &info.token;
        let dataset_id = match get_urn_from_string(&dataset_id) {
            Ok(dataset_id) => dataset_id,
            Err(_err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "urn not serializable"})),
                )
                    .into_response()
            }
        };
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": err.to_string()})),
                )
                    .into_response()
            }
        };
        match service.post_policy_offer(dataset_id, input, token.to_string()).await {
            Ok(policy) => (StatusCode::OK, Json(policy)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_business_delete_policy_offer(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        Path((catalog_id, dataset_id, policy_id)): Path<(String, String, String)>,
    ) -> impl IntoResponse {
        info!(
            "DELETE /gateway/api/catalogs/{}/datasets/{}/policies/{}",
            catalog_id, dataset_id, policy_id
        );
        let token = &info.token;
        let dataset_id = match get_urn_from_string(&dataset_id) {
            Ok(dataset_id) => dataset_id,
            Err(_err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "urn not serializable"})),
                )
                    .into_response()
            }
        };
        let policy_id = match get_urn_from_string(&policy_id) {
            Ok(policy_id) => policy_id,
            Err(_err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "urn not serializable"})),
                )
                    .into_response()
            }
        };
        match service.delete_policy_offer(dataset_id, policy_id, token.to_string()).await {
            Ok(_) => (StatusCode::OK).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_get_business_negotiation_requests(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
    ) -> impl IntoResponse {
        info!("GET /gateway/api/negotiation/business/requests");
        let token = &info.token;
        match service.get_business_negotiation_requests(token.to_string()).await {
            Ok(requests) => (StatusCode::OK, Json(requests)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_get_business_negotiation_request_by_id(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        Path(request_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /gateway/api/negotiation/business/requests/:request_id");
        let token = &info.token;
        let request_id = match get_urn_from_string(&request_id) {
            Ok(request_id) => request_id,
            Err(_err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "urn not serializable"})),
                )
                    .into_response()
            }
        };
        match service.get_business_negotiation_request_by_id(request_id, token.to_string()).await {
            Ok(requests) => (StatusCode::OK, Json(requests)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_get_customer_negotiation_requests(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        Path(participant_id): Path<String>,
    ) -> impl IntoResponse {
        info!(
            "GET /gateway/api/negotiation/consumer/{}/requests",
            participant_id
        );
        let token = &info.token;
        match service.get_consumer_negotiation_requests(participant_id, token.to_string()).await {
            Ok(requests) => (StatusCode::OK, Json(requests)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_get_consumer_negotiation_request_by_id(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        Path((participant_id, request_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!(
            "GET /gateway/api/negotiation/customer/{}/requests/{}",
            participant_id, request_id
        );
        let token = &info.token;
        let request_id = match get_urn_from_string(&request_id) {
            Ok(request_id) => request_id,
            Err(_err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "urn not serializable"})),
                )
                    .into_response()
            }
        };
        match service.get_consumer_negotiation_request_by_id(participant_id, request_id, token.to_string()).await {
            Ok(requests) => (StatusCode::OK, Json(requests)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_accept_request(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<RainbowBusinessAcceptanceRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /gateway/api/negotiation/rpc/accept");
        let token = &info.token;
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": err.body_text()})),
                )
                    .into_response()
            }
        };
        match service.accept_request(input, token.to_string()).await {
            Ok(res) => (StatusCode::ACCEPTED, Json(res)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_terminate_request(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<RainbowBusinessTerminationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /gateway/api/negotiation/rpc/terminate");
        let token = &info.token;
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": err.body_text()})),
                )
                    .into_response()
            }
        };
        match service.terminate_request(input, token.to_string()).await {
            Ok(res) => (StatusCode::ACCEPTED, Json(res)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_create_request(
        State(service): State<Arc<T>>,
        Extension(info): Extension<Arc<RequestInfo>>,
        input: Result<Json<RainbowBusinessNegotiationRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /gateway/api/negotiation/rpc/request");
        let token = &info.token;
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": err.body_text()})),
                )
                    .into_response()
            }
        };
        match service.create_request(input, token.to_string()).await {
            Ok(res) => (StatusCode::ACCEPTED, Json(res)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
    async fn handle_login(
        State(service): State<Arc<T>>,
        input: Result<Json<RainbowBusinessLoginRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /gateway/api/login");
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": err.body_text()})),
                )
                    .into_response()
            }
        };
        match service.login(input).await {
            Ok(uri) => (StatusCode::ACCEPTED, uri).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }

    async fn handle_login_poll(
        State(service): State<Arc<T>>,
        input: Result<Json<RainbowBusinessLoginRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /gateway/api/login/poll");
        let input = match input {
            Ok(input) => input.0,
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": err.body_text()})),
                )
                    .into_response()
            }
        };
        match service.login_poll(input).await {
            Ok(uri) => (StatusCode::ACCEPTED, Json(uri)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        }
    }
}
