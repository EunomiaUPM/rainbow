use crate::core::vc_request_service::vc_request_types::VCRequest;
use crate::core::vc_request_service::VCRequestTrait;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use rainbow_common::err::transfer_err::TransferErrorType::ProtocolBodyError;
use rainbow_common::protocol::transfer::transfer_request::TransferRequestMessage;
use rainbow_common::utils::get_urn_from_string;
use std::sync::Arc;
use tracing::info;

pub struct AuthorityRouter<T>
where
    T: VCRequestTrait + Send + Sync + 'static,
{
    authority_service: Arc<T>,
}

impl<T> AuthorityRouter<T>
where
    T: VCRequestTrait + Send + Sync + 'static,
{
    pub fn new(authority_service: Arc<T>) -> Self {
        Self { authority_service }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/authority/vc-requests", get(Self::handler_get_all_vc_requests))
            .route("/api/v1/authority/vc-requests/:vc_id", get(Self::handler_get_vc_request_by_id))
            .route("/api/v1/authority/vc-requests/:vc_id/validate", post(Self::handler_get_validate_vc_request))
            .route("/api/v1/authority/vc-requests/:vc_id/reject", post(Self::handler_get_reject_vc_request))
            .route("/api/v1/authority/vc-requests/request-credential", post(Self::handle_request_credential))
            .with_state(self.authority_service)
    }
    async fn handler_get_all_vc_requests(
        State(authority_service): State<Arc<T>>
    ) -> impl IntoResponse {
        info!("GET /api/v1/authority/vc-requests");
        match authority_service.get_all_vc_requests().await {
            Ok(payload) => (StatusCode::BAD_REQUEST, Json(payload)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }
    async fn handler_get_vc_request_by_id(
        State(authority_service): State<Arc<T>>,
        Path(vc_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/authority/vc-requests/{}", vc_id.clone());
        let vc_id = match get_urn_from_string(&vc_id) {
            Ok(vc_id) => vc_id,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match authority_service.get_vc_request_by_id(vc_id).await {
            Ok(payload) => (StatusCode::BAD_REQUEST, Json(payload)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }
    async fn handler_get_validate_vc_request(
        State(authority_service): State<Arc<T>>,
        Path(vc_id): Path<String>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/authority/vc-requests/{}/validate", vc_id.clone());
        let vc_id = match get_urn_from_string(&vc_id) {
            Ok(vc_id) => vc_id,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match authority_service.validate_vc_request_by_id(vc_id).await {
            Ok(payload) => (StatusCode::BAD_REQUEST, Json(payload)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }
    async fn handler_get_reject_vc_request(
        State(authority_service): State<Arc<T>>,
        Path(vc_id): Path<String>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/authority/vc-requests/{}/reject", vc_id.clone());
        let vc_id = match get_urn_from_string(&vc_id) {
            Ok(vc_id) => vc_id,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match authority_service.reject_vc_request_by_id(vc_id).await {
            Ok(payload) => (StatusCode::BAD_REQUEST, Json(payload)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }
    async fn handle_request_credential(
        State(authority_service): State<Arc<T>>,
        input: Result<Json<VCRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/authority/vc-requests/request-credential");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match authority_service.create_vc_request(input).await {
            Ok(payload) => (StatusCode::BAD_REQUEST, Json(payload)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }
}
